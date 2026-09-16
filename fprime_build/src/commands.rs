use crate::tree::Qualifier;
use crate::types::type_name;
use crate::util::{annotate_with_args, hex_literal, split_identifier, str_to_ident};
use proc_macro2::TokenStream;
use quote::quote;

pub fn command(cmd: &fprime_dictionary::Command) -> (Qualifier, TokenStream) {
    let (q, name) = split_identifier(&cmd.name);
    let args = cmd.formal_params.iter().map(|arg| {
        let name = str_to_ident(&arg.name);
        let ty = type_name(&arg.type_name);
        quote! { #name: #ty, }
    });

    let opcode = hex_literal(cmd.opcode);

    let def = quote! {
        #[fprime_command(opcode = #opcode)]
        pub fn #name(&self, #(#args)*) -> fprime_core::CmdResponse {}
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
