use fprime_dictionary::Value;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, quote};

pub fn value(v: &Value) -> TokenStream {
    match v {
        Value::Integer(i) => Literal::i64_unsuffixed(*i).into_token_stream(),
        Value::Float(f) => quote! { #f },
        Value::Bool(b) => quote! { #b },
        Value::String(s) => quote! { #s },
        Value::Array(values) => {
            let value_tokens = values.iter().map(value);
            quote! {
                {#(#value_tokens,)*}
            }
        }
        Value::Struct(hash_map) => {
            let member_tokens = hash_map.iter().map(|(k, v)| {
                let member_value = value(v);
                quote! {
                    #k: #member_value
                }
            });

            quote! {
                {#(#member_tokens,)*}
            }
        }
    }
}
