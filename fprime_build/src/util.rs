use crate::tree::Qualifier;
use convert_case::{Case, Casing};
use fprime_dictionary::{Command, Dictionary, FloatKind, IntegerKind, TypeDefinition, TypeName};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::str::FromStr;

fn type_name_size(dictionary: Dictionary, ty: TypeName) -> usize {
    match ty {
        TypeName::Integer { name } => match name {
            IntegerKind::U8 | IntegerKind::I8 => 1,
            IntegerKind::U16 | IntegerKind::I16 => 2,
            IntegerKind::U32 | IntegerKind::I32 => 4,
            IntegerKind::U64 | IntegerKind::I64 => 8,
        },
        TypeName::Float { name } => match name {
            FloatKind::F32 => 4,
            FloatKind::F64 => 8,
        },
        TypeName::Bool => 1,
        TypeName::String { size } => {
            // Size prefix
            type_name_size(dictionary, TypeName::QualifiedIdentifier {
                name: "FwSizeType".to_string()
            }) + (size as usize)
        }
        TypeName::QualifiedIdentifier { name } => {
            dictionary.type_definitions.iter().find(|d| d.qualified_name() == name)
        }
    }
}

fn type_definition_size(dictionary: Dictionary, ty: TypeDefinition) -> usize {
    match ty {
        TypeDefinition::Array(a) => (a.size as usize) * type_name_size(dictionary, a.element_type),
        TypeDefinition::Enum(e) => type_name_size(dictionary, e.representation_type),
        TypeDefinition::Struct(s) => s.members.iter().fold(0, |size, m| {
            size + (type_name_size(dictionary, m.type_name) * (m.size.unwrap_or(1) as usize))
        }),
        TypeDefinition::Alias(a) => type_name_size(dictionary, a.underlying_type),
    }
}

fn cmd_size(dictionary: Dictionary, cmd: &Command) -> usize {
    dictionary.type_definitions.iter().find_map(|f| match f {
        TypeDefinition::Alias(a) => {
            if a.qualified_name == "FwOpcodeType" {
                Some()
            }
        }
        _ => None,
    })
}

pub(crate) fn global_memory(dictionary: Dictionary) -> TokenStream {
    // Compute the upper bound scratch space needed
    // Scratch memory is used for telemetry, commands and parameters
    let max_size = 0usize;
    for d in dictionary.commands.iter() {}

    quote! {
        const __SCRATCH_SIZE: usize = 512;
        static mut __SCRATCH: [u8; __SCRATCH_SIZE] = [0x0; __SCRATCH_SIZE];
        static mut __TIME: [u8; fw::TimeValue::SIZE] = [0; fw::TimeValue::SIZE];
    }
}

pub(crate) fn qualified_identifier(qi: &str, name_kind: NameKind) -> (Qualifier, Ident) {
    let mut qn: Vec<&str> = qi.split('.').collect();

    let name = qn
        .pop()
        .expect(&format!("invalid qualified identifier: '{}'", qi));

    (
        qn.into_iter()
            .map(|q| format_name(NameKind::Module, q))
            .collect(),
        format_name(name_kind, name),
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

pub(crate) fn annotate_with_args(
    inner: TokenStream,
    annotation: &Option<String>,
    arg_annotations: Vec<(String, Option<String>)>,
) -> TokenStream {
    if arg_annotations.len() == 0 {
        return annotate(inner, annotation);
    }

    let brief = match annotation {
        None => "".to_string(),
        Some(annotation) => annotation.clone(),
    };

    let args = arg_annotations
        .iter()
        .map(|(name, annot)| match annot {
            None => format!(" * `{}`", name),
            Some(arg_a) => format!(" * `{}` - {}", name, arg_a),
        })
        .collect::<Vec<String>>()
        .join("\n");

    let annot = quote_doctest::doc_comment(format!("{}\n\n{}", brief, args));
    quote! {
        #annot
        #inner
    }
}

pub enum NameKind {
    Module,
    Definition,
    EnumConstant,
    StructMember,
    FormalParameter,
    Function,
}

pub(crate) fn format_name(kind: NameKind, name: &str) -> Ident {
    // TODO(tumbar) Add a compiler context to manage settings
    let case = match kind {
        NameKind::Definition => Case::Pascal,
        NameKind::EnumConstant => Case::Pascal,
        NameKind::StructMember => Case::Snake,
        NameKind::Module => Case::Snake,
        NameKind::FormalParameter => Case::Snake,
        NameKind::Function => Case::Snake,
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

pub(crate) fn hex_literal(value: u64) -> Literal {
    let hex_string = format!("0x{:X}", value);
    Literal::from_str(&hex_string).expect("Failed to parse hex literal string")
}
