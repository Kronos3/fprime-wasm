use crate::signature::{id_argument, parse_stub, return_type};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub(crate) fn parameter(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let id = id_argument(attr, "id")?;
    let stub = parse_stub(item)?;
    let value = return_type(&stub)?;

    let attrs = &stub.attrs;
    let vis = &stub.vis;
    let sig = &stub.sig;

    // Span the body onto the stub's own braces, see `command`
    let body = quote_spanned! { stub.block.span() =>
        {
            let mut __value: [u8; <#value as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)]
                core::mem::MaybeUninit::uninit().assume_init()
            };

            unsafe {
                parameter(#id, &mut __value)
            };

            <#value as Serializable>::deserialize(&__value)
        }
    };

    Ok(quote! {
        #(#attrs)*
        #vis #sig #body
    })
}
