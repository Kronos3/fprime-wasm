use crate::Qualifier;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(crate) fn qualified_identifier(qi: &str) -> (Qualifier, Ident) {
    let mut qn: Vec<&str> = qi.split('.').collect();

    let name = qn
        .pop()
        .expect(&format!("invalid qualified identifier: '{}'", qi));

    (
        qn.into_iter()
            .map(|q| format_name(NameKind::Module, q))
            .collect(),
        format_name(NameKind::Definition, name),
    )
}

pub(crate) fn annotate(inner: TokenStream, annotation: &Option<String>) -> TokenStream {
    match annotation {
        None => inner,
        Some(annotation) => {
            let annot = quote_doctest::doc_comment(annotation);
            quote! {
                #annot
                #inner
            }
        }
    }
}

pub enum NameKind {
    Module,
    Definition,
    EnumConstant,
    StructMember,
}

pub(crate) fn format_name(kind: NameKind, name: &str) -> Ident {
    // TODO(tumbar) Add a compiler context to manage settings
    let case = match kind {
        NameKind::Definition => Case::Pascal,
        NameKind::EnumConstant => Case::Pascal,
        NameKind::StructMember => Case::Snake,
        NameKind::Module => Case::Snake,
    };

    match name.to_case(case).as_str() {
        // Keywords
        name @ ("as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" |
        // Restricted
        "names" | "abstract" | "become" | "box" | "do" | "final" | "gen" | "macro" | "override" | "priv" | "try" | "typeof" | "unsized" | "virtual" | "yield") => {
            // Protect against Rust keyword overlap
            Ident::new_raw(name, Span::call_site())
        }
        name => Ident::new(name, Span::call_site())
    }
}
