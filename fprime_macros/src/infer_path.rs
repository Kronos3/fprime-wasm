//! A passthrough DSL for writing F Prime command sequences.
//!
//! Inside a function marked `#[fprime]` (or `#[fprime_main]`), a command
//! argument may name an enumerated constant on its own:
//!
//! ```ignore
//! Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
//! ```
//!
//! The receiver chain identifies the command in the dictionary, the command's
//! formal parameters give the expected type of every argument, and the
//! expansion replaces each bare constant with the full path of the enum that
//! declares it:
//!
//! ```ignore
//! Ref.dpDemo
//!     .Dp(crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE, 0, crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE);
//! ```

use fprime_dictionary::naming::{definition_path, definition_path_with_name, split_qualified_name, str_to_ident};
use fprime_dictionary::{Command, Dictionary, TypeDefinition, TypeName};
use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use syn::visit_mut::{self, VisitMut};
use syn::{Block, Expr, ExprPath, ExprStruct, Member, Path as SynPath};

/// Aliases followed before concluding the dictionary is cyclic.
const MAX_ALIAS_HOPS: usize = 32;

/// Nesting depth qualified inside a single command argument. Structs and arrays
/// in a dictionary cannot recurse, so this only bounds pathological input.
const MAX_DEPTH: usize = 32;
const COMPLETION_MARKER: &str = "raCompletionMarker";
const MISSING_DICTIONARY: &str = concat!(
    "the F Prime DSL needs a dictionary to resolve names against, but `",
    "FPRIME_DICTIONARY",
    "` is not set.\n",
    "Call `fprime_build::generate(\"<topology>Dictionary.json\")` from this crate's `build.rs`."
);

/// Rewrite the bare enumerated constants in `block` into fully qualified paths.
pub(crate) fn rewrite(attr: TokenStream, block: &mut Block, span: Span) -> syn::Result<()> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(attr, "this attribute takes no arguments"));
    }

    let Ok(dictionary) = std::env::var(fprime_dictionary::DICTIONARY_ENV) else {
        return Err(syn::Error::new(span, MISSING_DICTIONARY));
    };

    let resolver = resolver(Path::new(&dictionary)).map_err(|err| syn::Error::new(span, err))?;
    Dsl { resolver }.visit_block_mut(block);

    Ok(())
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
    /// Command name (`Ref.dpDemo.Dp`) to its index in `dictionary.commands`.
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

        Self { dictionary, commands }
    }

    fn command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name).map(|index| &self.dictionary.commands[*index])
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

                // Only a constant this enum declares is qualified. Any other
                // bare name belongs to the author -- a local, or a constant of
                // their own -- and is left alone.
                let Some(constant) = ty.enumerated_constants.iter().find(|c| c.name == name.lookup) else {
                    return;
                };

                let span = name.span;
                let constant = name.emit(&constant.name);
                let enum_path = definition_path(&ty.qualified_name);

                // Carry the argument's own attributes across, so that something
                // like `#[cfg(...)] IMMEDIATE` stays gated.
                let attrs = std::mem::take(&mut original.attrs);
                let mut qualified: ExprPath = parse(quote! { #enum_path::#constant }, span);
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
    ///
    /// `depth` is spent by `qualify` alone, so that one unit is one level of
    /// nesting however the level is reached.
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

struct Dsl<'a> {
    resolver: &'a Resolver,
}

impl VisitMut for Dsl<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::MethodCall(call) = expr
            && let Some(command) =
                command_name(&call.receiver, &call.method).and_then(|name| self.resolver.command(&name))
        {
            // Pairing stops at the shorter side, so a call with the wrong number
            // of arguments still gets the ones it does have qualified.
            for (arg, param) in call.args.iter_mut().zip(&command.formal_params) {
                self.resolver.qualify(arg, &param.type_name, 0);
            }
        }

        // Keep descending so that commands nested in arguments, closures and
        // blocks are qualified too.
        visit_mut::visit_expr_mut(self, expr);
    }
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

    let span = name.span;
    literal.path = parse(definition_path_with_name(qualified_name, name.emit(short_name)), span);
}

/// The dictionary name of the command `receiver.method` invokes, e.g. `Ref.dpDemo.Dp`.
fn command_name(receiver: &Expr, method: &Ident) -> Option<String> {
    let mut parts = vec![unraw(method)];
    let mut current = receiver;

    loop {
        match current {
            Expr::Field(field) => {
                let Member::Named(ident) = &field.member else {
                    return None;
                };

                parts.push(unraw(ident));
                current = &field.base;
            }
            Expr::Path(path) => {
                // The instance root may be written qualified (`crate::Ref`);
                // the dictionary only knows its final segment.
                let segment = path.path.segments.last()?;
                if !segment.arguments.is_none() {
                    return None;
                }

                parts.push(unraw(&segment.ident));
                break;
            }
            _ => return None,
        }
    }

    parts.reverse();
    Some(parts.join("."))
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
    ///
    /// While an editor is mid-name the written form is kept, so that its marker
    /// reaches the expansion; otherwise the dictionary's own spelling is used,
    /// which is what makes a name that collides with a keyword come out raw.
    fn emit(&self, dictionary_name: &str) -> Ident {
        if self.in_progress {
            self.written.clone()
        } else {
            str_to_ident(dictionary_name)
        }
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
        // Resolving the name without the marker is what keeps the expansion an
        // editor sees the same length as the real one up to the cursor.
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

/// Parse generated tokens, blaming the user's identifier for any later error.
fn parse<T: syn::parse::Parse>(tokens: TokenStream, span: Span) -> T {
    syn::parse2(respan(tokens, span)).expect("generated dictionary path should parse")
}

fn respan(tokens: TokenStream, span: Span) -> TokenStream {
    tokens
        .into_iter()
        .map(|token| match token {
            TokenTree::Group(group) => {
                let mut respanned = Group::new(group.delimiter(), respan(group.stream(), span));
                respanned.set_span(span);
                TokenTree::Group(respanned)
            }
            mut token => {
                token.set_span(span);
                token
            }
        })
        .collect()
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use quote::ToTokens;

    /// The dictionary the workspace tests share.
    const DICTIONARY: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fprime_dictionary/src/test/RefTopologyDictionary.json"
    );

    /// Apply the DSL to a sequence body and render the result as tokens.
    pub(crate) fn rewrite_block(body: &str) -> syn::Result<String> {
        let mut block = parse_block(body)?;

        let resolver = resolver(Path::new(DICTIONARY)).map_err(|err| syn::Error::new(Span::call_site(), err))?;
        Dsl { resolver }.visit_block_mut(&mut block);

        Ok(block.to_token_stream().to_string())
    }

    /// Render a body without rewriting it, so expectations can be written as
    /// ordinary Rust rather than as pre-spaced token strings.
    pub(crate) fn render_block(body: &str) -> syn::Result<String> {
        Ok(parse_block(body)?.to_token_stream().to_string())
    }

    fn parse_block(body: &str) -> syn::Result<Block> {
        syn::parse_str(&format!("{{ {} }}", body))
    }
}
