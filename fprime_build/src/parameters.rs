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
        #[fprime_parameter(id = #id)]
        pub fn #name(&self) -> #ty {}
    };

    (q, annotate(def, &prm.annotation))
}
