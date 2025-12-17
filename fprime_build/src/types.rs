use crate::util::NameKind::StructMember;
use crate::util::{annotate, format_name, qualified_identifier, NameKind};
use crate::Qualifier;
use fprime_dictionary::{
    AliasType, ArrayType, EnumType, FloatKind, IntegerKind, StructType, TypeDefinition, TypeName,
};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub(crate) fn type_name(tn: &TypeName) -> TokenStream {
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
            Some(size) => {
                let size_u = Literal::u32_unsuffixed(*size);
                quote! { heapless::String<#size_u> }
            }
        },
        TypeName::QualifiedIdentifier { name } => {
            let (qualifier, name) = qualified_identifier(name);
            quote! { crate::#(#qualifier::)*#name }
        }
    }
}

fn array_type_definition(ty: &ArrayType) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&ty.qualified_name);
    let tn = type_name(&ty.element_type);
    let size = Literal::u32_unsuffixed(ty.size);
    let arr_def = quote! {
        #[derive(Clone, Debug)]
        pub struct #name([#tn;#size]);
    };

    (q, annotate(arr_def, &ty.annotation))
}

fn enum_type_definition(ty: &EnumType) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&ty.qualified_name);
    let repr_ty = type_name(&ty.representation_type);
    let constants = ty.enumerated_constants.iter().map(|c| {
        let name = format_name(NameKind::EnumConstant, &c.name);
        let val = Literal::i64_unsuffixed(c.value);
        annotate(quote! { #name = #val, }, &c.annotation)
    });

    let enum_def = quote! {
        #[derive(Clone, Debug)]
        #[repr(#repr_ty)]
        pub enum #name {
            #(#constants)*
        }
    };

    (q, annotate(enum_def, &ty.annotation))
}

fn struct_type_definition(ty: &StructType) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&ty.qualified_name);
    let members = ty.members.iter().map(|member| {
        let name = format_name(StructMember, &member.name);
        let ty = type_name(&member.type_name);

        let inner = match member.size {
            None => quote! { pub #name: #ty, },
            Some(size) => {
                let size_u = Literal::u32_unsuffixed(size);
                quote! { pub #name: [#ty;#size_u], }
            }
        };
        annotate(inner, &member.annotation)
    });

    let struct_def = quote! {
        #[derive(Clone, Debug)]
        pub struct #name {
            #(#members)*
        }
    };

    (q, annotate(struct_def, &ty.annotation))
}

fn alias_type_definition(ty: &AliasType) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&ty.qualified_name);
    let tn = type_name(&ty.type_name);
    let alias_def = quote! {
        pub type #name = #tn;
    };

    (q, annotate(alias_def, &ty.annotation))
}

pub(crate) fn type_definition(ty: &TypeDefinition) -> (Qualifier, TokenStream) {
    match ty {
        TypeDefinition::Array(a) => array_type_definition(a),
        TypeDefinition::Enum(e) => enum_type_definition(e),
        TypeDefinition::Struct(s) => struct_type_definition(s),
        TypeDefinition::Alias(a) => alias_type_definition(a),
    }
}
