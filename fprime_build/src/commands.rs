use crate::types::type_name;
use crate::util::{annotate_with_args, format_name, hex_literal, qualified_identifier, NameKind};
use crate::Qualifier;
use fprime_dictionary::{EnumType, TypeName};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn command(
    cmd: &fprime_dictionary::Command,
    cmd_response: &EnumType,
) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&cmd.name, NameKind::Function);
    let args = cmd.formal_params.iter().map(|arg| {
        let name = format_name(NameKind::FormalParameter, &arg.name);
        match &arg.type_name {
            TypeName::String { .. } => quote! { #name: &str, },
            _ => {
                let ty = type_name(&arg.type_name);
                quote! { #name: #ty, }
            }
        }
    });

    let arg_sizes = cmd.formal_params.iter().map(|arg| match &arg.type_name {
        TypeName::String { .. } => {
            let ty = type_name(&arg.type_name);
            quote! { <#ty as Serializable>::SIZE }
        }
        _ => {
            let ty = type_name(&arg.type_name);
            quote! { #ty::SIZE }
        }
    });

    let ser = cmd.formal_params.iter().map(|arg| {
        let name = format_name(NameKind::FormalParameter, &arg.name);
        let value = match &arg.type_name {
            TypeName::String { size } => {
                let ty = type_name(&arg.type_name);
                // TODO(tumbar) Make string commanding configurable
                // Panic if string doesn't fit
                // quote! { <#ty as TryFrom<&str>>::try_from(#name).unwrap() }

                // Truncate string (fast)
                let size_lit = Literal::u32_unsuffixed(*size);
                quote! { <#ty as StrTruncate<#size_lit>>::truncate(#name) }
            }
            _ => {
                quote! { #name }
            }
        };

        quote! {
            #value.serialize_to(&mut __encoded, &mut __offset);
        }
    });

    let opcode = hex_literal(cmd.opcode);
    let response_repr_ty = type_name(&cmd_response.representation_type);

    let def = quote! {
        pub fn #name(#(#args)*) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE #(+ #arg_sizes)*] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = #opcode;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            #(#ser)*

            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe {
                core::mem::transmute(res as #response_repr_ty)
            }
        }
    };

    (
        q,
        annotate_with_args(
            def,
            &cmd.annotation,
            cmd.formal_params
                .iter()
                .map(|arg| (arg.name.clone(), arg.annotation.clone()))
                .collect(),
        ),
    )
}
