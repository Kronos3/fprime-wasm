use crate::types::type_name;
use crate::util::{annotate, format_name, hex_literal, qualified_identifier, NameKind};
use crate::Qualifier;
use fprime_dictionary::TypeName;
use proc_macro2::TokenStream;
use quote::quote;

pub fn command(cmd: &fprime_dictionary::Command) -> (Qualifier, TokenStream) {
    let (q, name) = qualified_identifier(&cmd.name);
    let args = cmd.formal_params.iter().map(|arg| {
        let name = format_name(NameKind::FormalParameter, &arg.name);
        let ty = type_name(&arg.type_name);
        quote! { #name: #ty, }
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

        quote! {
            #name.serialize_to(&mut __encoded, &mut __offset);
        }
    });

    let opcode = hex_literal(cmd.opcode);

    let def = quote! {
        pub fn #name(#(#args)*) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;

            let mut __encoded: [u8; crate::FwOpcodeType::SIZE #(+ #arg_sizes)*] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = #opcode;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            #(#ser)*

            crate::fw::CmdResponse::Ok
        }
    };

    (q, annotate(def, &cmd.annotation))
}
