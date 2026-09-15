use crate::tree::Qualifier;
use fprime_dictionary::{Command, Dictionary, FloatKind, IntegerKind, TypeDefinition, TypeName};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::str::FromStr;

fn type_name_size(dictionary: &Dictionary, ty: &TypeName) -> usize {
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
            type_name_size(
                &dictionary,
                &TypeName::QualifiedIdentifier {
                    name: "FwSizeType".to_string(),
                },
            ) + (*size as usize)
        }
        TypeName::QualifiedIdentifier { name } => type_definition_size(
            &dictionary,
            &dictionary
                .type_definitions
                .get(&*name)
                .expect("Expected type definition"),
        ),
    }
}

fn type_definition_size(dictionary: &Dictionary, ty: &TypeDefinition) -> usize {
    match ty {
        TypeDefinition::Array(a) => (a.size as usize) * type_name_size(dictionary, &a.element_type),
        TypeDefinition::Enum(e) => type_name_size(dictionary, &e.representation_type),
        TypeDefinition::Struct(s) => s.members.iter().fold(0, |size, m| {
            size + (type_name_size(dictionary, &m.type_name) * (m.size.unwrap_or(1) as usize))
        }),
        TypeDefinition::Alias(a) => type_name_size(dictionary, &a.underlying_type),
    }
}

fn cmd_size(dictionary: &Dictionary, cmd: &Command) -> usize {
    let opcode_size = type_definition_size(
        &dictionary,
        dictionary.type_definitions.get("FwOpcodeType").unwrap(),
    );

    opcode_size
        + cmd.formal_params.iter().fold(0, |size, param| {
            type_name_size(dictionary, &param.type_name) + size
        })
}

pub(crate) fn global_memory(dictionary: &Dictionary) -> TokenStream {
    // Compute the upper bound scratch space needed
    // Scratch memory is used for telemetry, commands and parameters
    // TODO(tumbar) Add parameters
    let max_size = dictionary
        .commands
        .iter()
        .map(|cmd| cmd_size(&dictionary, &cmd))
        .chain(
            dictionary
                .telemetry_channels
                .iter()
                .map(|tlm| type_name_size(&dictionary, &tlm.type_name)),
        )
        .max()
        .unwrap_or(0);

    quote! {
        const __SCRATCH_SIZE: usize = #max_size;
        static mut __SCRATCH: [u8; __SCRATCH_SIZE] = [0x0; __SCRATCH_SIZE];
        static mut __TIME: [u8; crate::Defs::Fw::TimeValue::SIZE] = [0; crate::Defs::Fw::TimeValue::SIZE];
    }
}

pub(crate) fn split_identifier(qi: &str, name_kind: NameKind) -> (Qualifier, Ident) {
    let mut qn: Vec<&str> = qi.split('.').collect();

    let name = qn
        .pop()
        .expect(&format!("invalid qualified identifier: '{}'", qi));

    (
        qn.iter().map(|s| s.to_string()).collect(),
        str_to_ident(&format_name(name_kind, name)),
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
    Constant,
}

pub(crate) fn format_name(_kind: NameKind, name: &str) -> String {
    // TODO(tumbar) Add a compiler context to manage settings
    // let case = match kind {
    //     NameKind::Definition => Case::Pascal,
    //     NameKind::EnumConstant => Case::Pascal,
    //     NameKind::StructMember => Case::Snake,
    //     NameKind::Module => Case::Snake,
    //     NameKind::FormalParameter => Case::Snake,
    //     NameKind::Function => Case::Snake,
    // };
    //
    // name.to_case(case)

    name.to_string()
}

pub(crate) fn str_to_ident(name: &str) -> Ident {
    match name {
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
