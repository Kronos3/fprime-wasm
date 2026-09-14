use crate::tree::Qualifier;
use crate::types::type_name;
use crate::util::{NameKind, annotate_with_args, format_name, hex_literal, qualified_identifier};
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

    let ser = cmd.formal_params.iter().map(|arg| {
        let name = format_name(NameKind::FormalParameter, &arg.name);
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
    let scratch_ns1 = (0..q.len()).map(|_| quote!(super::));
    let scratch_ns2 = (0..q.len()).map(|_| quote!(super::));

    let def = quote! {
        pub fn #name(#(#args)*) -> crate::fw::CmdResponse {
            let __encoded = unsafe {
                let ptr = (&raw mut #(#scratch_ns1)* __SCRATCH) as *mut u8;
                let len = #(#scratch_ns2)* __SCRATCH_SIZE;

                core::slice::from_raw_parts_mut(ptr, len)
            };

            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = #opcode;
            __opcode.serialize_to(__encoded, &mut __offset);
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
