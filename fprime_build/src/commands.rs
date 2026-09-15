use crate::tree::Qualifier;
use crate::types::type_name;
use crate::util::{annotate_with_args, hex_literal, split_identifier, str_to_ident};
use fprime_dictionary::{EnumType, TypeName};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn command(
    cmd: &fprime_dictionary::Command,
    cmd_response: &EnumType,
) -> (Qualifier, TokenStream) {
    let (q, name) = split_identifier(&cmd.name);
    let args = cmd.formal_params.iter().map(|arg| {
        let name = str_to_ident(&arg.name);
        match &arg.type_name {
            TypeName::String { .. } => quote! { #name: &str, },
            _ => {
                let ty = type_name(&arg.type_name);
                quote! { #name: #ty, }
            }
        }
    });

    let ser = cmd.formal_params.iter().map(|arg| {
        let name = str_to_ident(&arg.name);
        let value = match &arg.type_name {
            TypeName::String { size } => {
                let ty = type_name(&arg.type_name);
                // TODO(tumbar) Make string commanding configurable
                // Currently we accept &str and truncate size to fit

                // Truncate string
                let size_lit = Literal::u32_unsuffixed(*size);
                quote! { <#ty as StrTruncate<#size_lit>>::truncate(#name) }
            }
            _ => {
                quote! { #name }
            }
        };

        quote! {
            #value.serialize_to(__encoded, &mut __offset);
        }
    });

    let opcode = hex_literal(cmd.opcode);
    let response_repr_ty = type_name(&cmd_response.representation_type);

    let def = quote! {
        pub fn #name(&self, #(#args)*) -> super::Defs::Fw::CmdResponse {
            let __encoded = unsafe {
                let ptr = (&raw mut __SCRATCH) as *mut u8;
                let len = __SCRATCH_SIZE;

                core::slice::from_raw_parts_mut(ptr, len)
            };

            let mut __offset: usize = 0;
            let __opcode: super::Defs::FwOpcodeType = #opcode;
            __opcode.serialize_to(__encoded, &mut __offset);
            #(#ser)*

            let res = unsafe { command(&__encoded[0..__offset]) };
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
