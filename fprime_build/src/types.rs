use fprime_dictionary::{
    AliasType, ArrayType, EnumType, FloatKind, IntegerKind, StructType, TypeDefinition, TypeName,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(crate) fn type_name(tn: TypeName) -> TokenStream {
    match tn {
        TypeName::Integer { name } => match name {
            IntegerKind::U8 => quote! { u8 },
            IntegerKind::I8 => quote! { i8 },
            IntegerKind::U16 => quote! { u16 },
            IntegerKind::I16 => quote! { i16 },
            IntegerKind::U32 => quote! { u32 },
            IntegerKind::I32 => quote! { i32 },
            IntegerKind::U64 => quote! { u64 },
            IntegerKind::I64 => quote! { i64 },
        },
        TypeName::Float { name } => match name {
            FloatKind::F32 => quote! { f32 },
            FloatKind::F64 => quote! { f64 },
        },
        TypeName::Bool => quote! { bool },
        TypeName::String { size } => match size {
            None => quote! { heapless::String<crate::DEFAULT_STRING_SIZE> },
            Some(size) => quote! { heapless::String<#size> },
        },
        TypeName::QualifiedIdentifier { name } => {
            let qn: Vec<Ident> = name
                .split(".")
                .map(|q| Ident::new(q, Span::call_site()))
                .collect();
            quote! { crate [#(::#qn)*] }
        }
    }
}

fn array_type_definition(ty: ArrayType) -> TokenStream {

}

fn enum_type_definition(ty: EnumType) -> TokenStream {}

fn struct_type_definition(ty: StructType) -> TokenStream {}

fn alias_type_definition(ty: AliasType) -> TokenStream {}

pub(crate) fn type_definition(ty: TypeDefinition) -> TokenStream {
    match ty {
        TypeDefinition::Array(a) => array_type_definition(a),
        TypeDefinition::Enum(e) => enum_type_definition(e),
        TypeDefinition::Struct(s) => struct_type_definition(s),
        TypeDefinition::Alias(a) => alias_type_definition(a),
    }
}
