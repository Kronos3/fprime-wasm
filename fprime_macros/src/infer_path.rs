//! A passthrough DSL for writing F Prime command sequences.
//!
//! Inside a function marked `#[fprime]`, a command argument may name an
//! enumerated constant on its own:
//!
//! ```ignore
//! Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
//! ```
//!
//! This gets expanded into:
//!
//! ```ignore
//! Ref.dpDemo
//!     .Dp(crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE, 0, crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE);
//! ```

use fprime_dictionary::konst::{ENCODE_SUFFIX, SIZE_SUFFIX};
use fprime_dictionary::naming::{
    definition_path, definition_path_with_name, konst_path, split_qualified_name, str_to_ident,
};
use fprime_dictionary::{Command, Dictionary, TypeDefinition, TypeName};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use syn::{Expr, ExprPath, ExprStruct, Lit, Member, Path as SynPath, UnOp};

/// Aliases followed before concluding the dictionary is cyclic.
const MAX_ALIAS_HOPS: usize = 32;

/// Nesting depth qualified inside a single command argument
const MAX_DEPTH: usize = 32;
const COMPLETION_MARKER: &str = "raCompletionMarker";
const MISSING_DICTIONARY: &str = concat!(
    "the F Prime DSL needs a dictionary to resolve names against, but `",
    "FPRIME_DICTIONARY",
    "` is not set.\n",
    "Call `fprime_build::generate(\"<topology>Dictionary.json\")` from this crate's `build.rs`."
);

/// Rewrite the bare enumerated constants in `item` into fully qualified paths.
pub(crate) fn rewrite(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(
            attr,
            "this attribute takes no arguments",
        ));
    }

    let span = signature_span(&item);

    let Ok(dictionary) = std::env::var(fprime_dictionary::DICTIONARY_ENV) else {
        return Err(syn::Error::new(span, MISSING_DICTIONARY));
    };

    let resolver = resolver(Path::new(&dictionary)).map_err(|err| syn::Error::new(span, err))?;

    let qualified = resolver.qualify_calls(item);

    Ok(resolver.const_encode_calls(qualified))
}

fn signature_span(item: &TokenStream) -> Span {
    let mut first = None;
    let mut after_fn = false;

    for tree in item.clone() {
        first = first.or(Some(tree.span()));

        match &tree {
            TokenTree::Ident(name) if after_fn => return name.span(),
            TokenTree::Ident(keyword) if keyword == "fn" => after_fn = true,
            _ => {}
        }
    }

    first.unwrap_or_else(Span::call_site)
}

/// Dictionaries are parsed once per compilation and shared by every expansion.
fn resolver(dictionary: &Path) -> Result<&'static Resolver, String> {
    static CACHE: OnceLock<Mutex<HashMap<PathBuf, &'static Resolver>>> = OnceLock::new();

    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().unwrap_or_else(|err| err.into_inner());

    if let Some(resolver) = cache.get(dictionary) {
        return Ok(resolver);
    }

    let parsed = fprime_dictionary::try_parse(dictionary)?;
    let resolver: &'static Resolver = Box::leak(Box::new(Resolver::new(parsed)));
    cache.insert(dictionary.to_path_buf(), resolver);

    Ok(resolver)
}

struct Resolver {
    dictionary: Dictionary,
    commands: HashMap<String, usize>,
}

impl Resolver {
    fn new(dictionary: Dictionary) -> Self {
        let commands = dictionary
            .commands
            .iter()
            .enumerate()
            .map(|(index, command)| (command.name.clone(), index))
            .collect();

        Self {
            dictionary,
            commands,
        }
    }

    fn command(&self, name: &str) -> Option<&Command> {
        self.commands
            .get(name)
            .map(|index| &self.dictionary.commands[*index])
    }

    /// Follow aliases down to the definition that carries the constants or members.
    fn definition(&self, ty: &TypeName) -> Option<&TypeDefinition> {
        let TypeName::QualifiedIdentifier { name } = ty else {
            return None;
        };

        let mut definition = self.dictionary.type_definitions.get(name)?;
        for _ in 0..MAX_ALIAS_HOPS {
            let TypeDefinition::Alias(alias) = definition else {
                return Some(definition);
            };

            let TypeName::QualifiedIdentifier { name } = &alias.underlying_type else {
                return None;
            };

            definition = self.dictionary.type_definitions.get(name)?;
        }

        None
    }

    fn qualify_calls(&self, tokens: TokenStream) -> TokenStream {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        let mut qualified = TokenStream::new();
        let mut index = 0;

        while index < trees.len() {
            let begins_a_chain = match index.checked_sub(1).map(|previous| &trees[previous]) {
                Some(TokenTree::Punct(punct)) => !matches!(punct.as_char(), '.' | ':'),
                _ => true,
            };

            if begins_a_chain
                && let Some(call) = self.command_call(&trees, index)
                && let Some(TokenTree::Group(args)) = trees.get(call.args)
            {
                qualified.extend(trees[index..call.args].iter().cloned());
                qualified.extend([TokenTree::Group(self.qualify_args(args, call.command))]);
                index = call.args + 1;
                continue;
            }

            match &trees[index] {
                // A command in a block, a closure or an argument is still a command.
                TokenTree::Group(group) => {
                    let mut descended =
                        Group::new(group.delimiter(), self.qualify_calls(group.stream()));
                    descended.set_span(group.span());
                    qualified.extend([TokenTree::Group(descended)]);
                }
                tree => qualified.extend([tree.clone()]),
            }

            index += 1;
        }

        qualified
    }

    fn command_call(&self, trees: &[TokenTree], start: usize) -> Option<CommandCall<'_>> {
        let mut index = start;
        let mut root = ident_at(trees, index)?;

        index += 1;
        while let Some(segment) = path_separator_at(trees, index) {
            root = ident_at(trees, segment)?;
            index = segment + 1;
        }

        let mut parts = vec![unraw(root)];
        while let Some(field) = field_separator_at(trees, index) {
            parts.push(unraw(ident_at(trees, field)?));
            index = field + 1;
        }

        let TokenTree::Group(args) = trees.get(index)? else {
            return None;
        };

        if args.delimiter() != Delimiter::Parenthesis {
            return None;
        }

        Some(CommandCall {
            command: self.command(&parts.join("."))?,
            args: index,
        })
    }

    fn qualify_args(&self, args: &Group, command: &Command) -> Group {
        let mut rewritten = TokenStream::new();

        for (position, argument) in arguments(args.stream()).into_iter().enumerate() {
            let written = self.qualify_calls(argument.tokens);

            let expected = command.formal_params.get(position).and_then(|param| {
                syn::parse2::<Expr>(written.clone())
                    .ok()
                    .map(|expr| (param, expr))
            });

            match expected {
                Some((param, mut expr)) => {
                    self.qualify(&mut expr, &param.type_name, 0);
                    rewritten.extend(expr.into_token_stream());
                }
                None => rewritten.extend(written),
            }

            rewritten.extend(argument.separator);
        }

        let mut qualified = Group::new(args.delimiter(), rewritten);
        qualified.set_span(args.span());
        qualified
    }

    /// Replace every command call whose arguments are all compile-time constants
    /// with a reference to its encoded `Fw::ComBuffer`.
    fn const_encode_calls(&self, tokens: TokenStream) -> TokenStream {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        let mut encoded = TokenStream::new();
        let mut index = 0;

        while index < trees.len() {
            let begins_a_chain = match index.checked_sub(1).map(|previous| &trees[previous]) {
                Some(TokenTree::Punct(punct)) => !matches!(punct.as_char(), '.' | ':'),
                _ => true,
            };

            if begins_a_chain
                && let Some(call) = self.command_call(&trees, index)
                && let Some(TokenTree::Group(args)) = trees.get(call.args)
                && let Some(buffer) = self.const_encoded_call(
                    call.command,
                    args,
                    trees[index..=call.args].iter().cloned().collect(),
                )
            {
                encoded.extend(buffer);
                index = call.args + 1;
                continue;
            }

            match &trees[index] {
                TokenTree::Group(group) => {
                    let mut descended =
                        Group::new(group.delimiter(), self.const_encode_calls(group.stream()));
                    descended.set_span(group.span());
                    encoded.extend([TokenTree::Group(descended)]);
                }
                tree => encoded.extend([tree.clone()]),
            }

            index += 1;
        }

        encoded
    }

    /// The const-encoded form of `command` applied to `args`, or `None` if this
    /// call has to keep the runtime `__SCRATCH` path.
    fn const_encoded_call(
        &self,
        command: &Command,
        args: &Group,
        original: TokenStream,
    ) -> Option<TokenStream> {
        if !self.dictionary.const_encodable(command) {
            return None;
        }

        let parsed = arguments(args.stream());

        // A call with the wrong arity is left alone, so that the accessor is what
        // reports it against the user's own argument list.
        if parsed.len() != command.formal_params.len() {
            return None;
        }

        for (param, argument) in command.formal_params.iter().zip(&parsed) {
            let expr = syn::parse2::<Expr>(argument.tokens.clone()).ok()?;

            if !self.is_const_argument(&expr, &param.type_name) {
                return None;
            }
        }

        let size = konst_path(&command.name, SIZE_SUFFIX);
        let encode = konst_path(&command.name, ENCODE_SUFFIX);
        let args = args.stream();

        Some(quote! {
            {
                const __FPRIME_LEN: usize = #size(#args);
                const __FPRIME_CMD: [u8; __FPRIME_LEN] = #encode::<__FPRIME_LEN>(#args);

                if false {
                    #original;
                }

                unsafe { fprime_core::command(&__FPRIME_CMD) }
            }
        })
    }

    /// Whether `expr` may appear in the `const` initialiser of a const-encoded
    /// call.
    fn is_const_argument(&self, expr: &Expr, expected: &TypeName) -> bool {
        self.is_const_argument_at(expr, expected, 0)
    }

    fn is_const_argument_at(&self, expr: &Expr, expected: &TypeName, depth: usize) -> bool {
        if depth > MAX_DEPTH {
            return false;
        }

        match self.dictionary.resolved_type(expected) {
            Some(TypeName::String { .. }) => {
                matches!(expr, Expr::Lit(lit) if matches!(lit.lit, Lit::Str(_)))
            }
            Some(TypeName::Bool) => {
                matches!(expr, Expr::Lit(lit) if matches!(lit.lit, Lit::Bool(_)))
            }
            Some(TypeName::Integer { .. } | TypeName::Float { .. }) => is_numeric_literal(expr),
            Some(TypeName::QualifiedIdentifier { name }) => {
                match self.dictionary.type_definitions.get(name) {
                    Some(TypeDefinition::Enum(_)) => is_qualified_definition(expr),
                    Some(TypeDefinition::Array(ty)) => {
                        self.is_const_elements(expr, &ty.element_type, ty.size, depth)
                    }
                    Some(TypeDefinition::Struct(ty)) => {
                        let Expr::Struct(literal) = expr else {
                            return false;
                        };

                        // A struct literal that borrows the rest of its fields
                        // from somewhere is not something we can evaluate.
                        if literal.rest.is_some() || !is_qualified_path(&literal.path) {
                            return false;
                        }

                        // Every member has to be given, and given a constant.
                        ty.members.len() == literal.fields.len()
                            && ty.members.iter().all(|member| {
                                let field = literal.fields.iter().find(|field| {
                                    matches!(&field.member, Member::Named(name)
                                        if unraw(name) == member.name)
                                });

                                let Some(field) = field else {
                                    return false;
                                };

                                match member.size {
                                    None => self.is_const_argument_at(
                                        &field.expr,
                                        &member.type_name,
                                        depth + 1,
                                    ),
                                    Some(size) => self.is_const_elements(
                                        &field.expr,
                                        &member.type_name,
                                        size,
                                        depth,
                                    ),
                                }
                            })
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// An array of exactly `size` constants of type `element`, written either
    /// element by element or as a repeat.
    fn is_const_elements(&self, expr: &Expr, element: &TypeName, size: u32, depth: usize) -> bool {
        match expr {
            Expr::Array(array) => {
                array.elems.len() == size as usize
                    && array
                        .elems
                        .iter()
                        .all(|elem| self.is_const_argument_at(elem, element, depth + 1))
            }
            Expr::Repeat(repeat) => {
                // The length has to be a plain literal for us to know it matches.
                let Expr::Lit(lit) = repeat.len.as_ref() else {
                    return false;
                };
                let Lit::Int(len) = &lit.lit else {
                    return false;
                };

                len.base10_parse::<u32>().is_ok_and(|len| len == size)
                    && self.is_const_argument_at(&repeat.expr, element, depth + 1)
            }
            _ => false,
        }
    }

    /// Qualify `expr` against the type the dictionary expects there. An
    /// expression the dictionary has nothing to say about is left untouched.
    fn qualify(&self, expr: &mut Expr, expected: &TypeName, depth: usize) {
        if depth > MAX_DEPTH {
            return;
        }

        match self.definition(expected) {
            Some(TypeDefinition::Enum(ty)) => {
                let Expr::Path(original) = expr else {
                    return;
                };

                let Some(name) = bare_name(original.qself.as_ref(), &original.path) else {
                    return;
                };

                let Some(constant) = ty
                    .enumerated_constants
                    .iter()
                    .find(|c| c.name == name.lookup)
                else {
                    return;
                };

                let constant = name.emit(&constant.name);
                let enum_path = definition_path(&ty.qualified_name);

                let attrs = std::mem::take(&mut original.attrs);
                let mut qualified: ExprPath = parse(quote! { #enum_path::#constant });
                qualified.attrs = attrs;

                *expr = Expr::Path(qualified);
            }
            Some(TypeDefinition::Struct(ty)) => {
                let Expr::Struct(literal) = expr else {
                    return;
                };

                qualify_struct_path(literal, &ty.qualified_name);

                for field in literal.fields.iter_mut() {
                    let Member::Named(ident) = &field.member else {
                        continue;
                    };

                    let name = unraw(ident);
                    let Some(member) = ty.members.iter().find(|member| member.name == name) else {
                        continue;
                    };

                    match member.size {
                        None => self.qualify(&mut field.expr, &member.type_name, depth + 1),
                        Some(_) => self.qualify_elements(&mut field.expr, &member.type_name, depth),
                    }
                }
            }
            Some(TypeDefinition::Array(ty)) => self.qualify_elements(expr, &ty.element_type, depth),
            _ => {}
        }
    }

    /// Qualify every element of an array literal, or the repeated element of `[x; n]`.
    fn qualify_elements(&self, expr: &mut Expr, element: &TypeName, depth: usize) {
        match expr {
            Expr::Array(array) => {
                for element_expr in array.elems.iter_mut() {
                    self.qualify(element_expr, element, depth + 1);
                }
            }
            Expr::Repeat(repeat) => self.qualify(&mut repeat.expr, element, depth + 1),
            _ => {}
        }
    }
}

struct CommandCall<'a> {
    command: &'a Command,
    args: usize,
}

struct Argument {
    tokens: TokenStream,
    separator: Option<TokenTree>,
}

/// Split an argument list on its commas, keeping them.
/// A numeric literal, or a negated one: `-1` parses as a unary negation.
fn is_numeric_literal(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(lit) => matches!(lit.lit, Lit::Int(_) | Lit::Float(_)),
        Expr::Unary(unary) => matches!(unary.op, UnOp::Neg(_)) && is_numeric_literal(&unary.expr),
        _ => false,
    }
}

/// A path this pass qualified against the dictionary, which is rooted at
/// `crate`. A bare `SOME_NAME` is not one: it could be a local, and a user path
/// we did not write cannot be assumed to name a `const`.
fn is_qualified_definition(expr: &Expr) -> bool {
    let Expr::Path(path) = expr else {
        return false;
    };

    path.qself.is_none() && is_qualified_path(&path.path)
}

/// A path rooted at `crate`, which is the shape this pass writes when it
/// qualifies a name against the dictionary.
fn is_qualified_path(path: &SynPath) -> bool {
    path.segments.len() > 1
        && path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "crate")
}

fn arguments(tokens: TokenStream) -> Vec<Argument> {
    let mut arguments = Vec::new();
    let mut current = TokenStream::new();

    for tree in tokens {
        match &tree {
            TokenTree::Punct(punct) if punct.as_char() == ',' => {
                arguments.push(Argument {
                    tokens: std::mem::take(&mut current),
                    separator: Some(tree),
                });
            }
            _ => current.extend([tree]),
        }
    }

    // An argument list that ends in a comma has no argument after it, and an
    // empty one has no arguments at all.
    if !current.is_empty() {
        arguments.push(Argument {
            tokens: current,
            separator: None,
        });
    }

    arguments
}

fn ident_at(trees: &[TokenTree], index: usize) -> Option<&Ident> {
    match trees.get(index) {
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => None,
    }
}

/// Where the name after a `::` at `index` would be.
fn path_separator_at(trees: &[TokenTree], index: usize) -> Option<usize> {
    let (TokenTree::Punct(first), TokenTree::Punct(second)) =
        (trees.get(index)?, trees.get(index + 1)?)
    else {
        return None;
    };

    (first.as_char() == ':' && second.as_char() == ':').then_some(index + 2)
}

/// Where the name after a `.` at `index` would be.
fn field_separator_at(trees: &[TokenTree], index: usize) -> Option<usize> {
    let TokenTree::Punct(punct) = trees.get(index)? else {
        return None;
    };

    (punct.as_char() == '.').then_some(index + 1)
}

/// Replace an unqualified struct literal path with the path of the dictionary
/// struct it stands for, e.g. `ChoicePair { .. }` becomes `crate::Defs::Ref::ChoicePair { .. }`.
fn qualify_struct_path(literal: &mut ExprStruct, qualified_name: &str) {
    let Some(name) = bare_name(literal.qself.as_ref(), &literal.path) else {
        return;
    };

    let (_, short_name) = split_qualified_name(qualified_name);
    if name.lookup != short_name {
        return;
    }

    literal.path = parse(definition_path_with_name(
        qualified_name,
        name.emit(short_name),
    ));
}

/// A path that is nothing but a bare identifier, which may be one an editor is
/// partway through typing.
struct BareName {
    /// The name to look up: what the author has actually typed, with any editor
    /// marker taken back out.
    lookup: String,
    /// The identifier as written, marker and all.
    written: Ident,
    span: Span,
    /// Whether an editor is asking about this name rather than the author having
    /// finished it.
    in_progress: bool,
}

impl BareName {
    /// The identifier to emit for a dictionary name this resolved to.
    fn emit(&self, dictionary_name: &str) -> Ident {
        if self.in_progress {
            return self.written.clone();
        }

        let mut ident = str_to_ident(dictionary_name);
        ident.set_span(self.span);

        ident
    }
}

fn bare_name(qself: Option<&syn::QSelf>, path: &SynPath) -> Option<BareName> {
    if qself.is_some() || path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }

    let segment = path.segments.first()?;
    if !segment.arguments.is_none() {
        return None;
    }

    let written = unraw(&segment.ident);
    let in_progress = written.contains(COMPLETION_MARKER);

    Some(BareName {
        lookup: if in_progress {
            written.replace(COMPLETION_MARKER, "")
        } else {
            written
        },
        written: segment.ident.clone(),
        span: segment.ident.span(),
        in_progress,
    })
}

/// The identifier's name without the `r#` a keyword-named item carries, so it
/// compares equal to the plain name the dictionary stores.
fn unraw(ident: &Ident) -> String {
    let name = ident.to_string();
    name.strip_prefix("r#").map(str::to_string).unwrap_or(name)
}

fn parse<T: syn::parse::Parse>(tokens: TokenStream) -> T {
    syn::parse2(tokens).expect("generated dictionary path should parse")
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    /// The dictionary the workspace tests share.
    const DICTIONARY: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fprime_dictionary/src/test/RefTopologyDictionary.json"
    );

    /// Apply the DSL to a sequence body and render the result as tokens.
    pub(crate) fn rewrite_block(body: &str) -> syn::Result<String> {
        let resolver = resolver(Path::new(DICTIONARY))
            .map_err(|err| syn::Error::new(Span::call_site(), err))?;

        Ok(resolver.qualify_calls(syn::parse_str(body)?).to_string())
    }

    /// Both passes, in the order `rewrite` runs them: qualify, then const-encode.
    pub(crate) fn const_encode_block(body: &str) -> syn::Result<String> {
        let resolver = resolver(Path::new(DICTIONARY))
            .map_err(|err| syn::Error::new(Span::call_site(), err))?;

        let qualified = resolver.qualify_calls(syn::parse_str(body)?);

        Ok(resolver.const_encode_calls(qualified).to_string())
    }

    /// Render a body without rewriting it, so expectations can be written as
    /// ordinary Rust rather than as pre-spaced token strings.
    pub(crate) fn render_block(body: &str) -> syn::Result<String> {
        Ok(syn::parse_str::<TokenStream>(body)?.to_string())
    }
}
