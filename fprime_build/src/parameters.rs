use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    tree::Qualifier,
    types::type_name,
    util::{annotate, hex_literal, split_identifier},
};

pub fn parameter(prm: &fprime_dictionary::Parameter) -> (Qualifier, TokenStream) {
    let (q, name) = split_identifier(&prm.name);
    let ty = type_name(&prm.type_name);

    let id = hex_literal(prm.id);

    let def = quote! {
        pub fn #name(&self) -> #ty {
            let mut value_buf: [u8; <#ty as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            unsafe {
                parameter(#id, &mut value_buf)
            };

            <#ty as Serializable>::deserialize(&value_buf)
        }
    };

    (q, annotate(def, &prm.annotation))
}
