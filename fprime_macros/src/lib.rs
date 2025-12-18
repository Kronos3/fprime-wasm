use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, ItemEnum};

/// Returns the repr integer type as a string (e.g. "u8", "i32") if present.
fn enum_repr_type_name(attrs: &[syn::Attribute]) -> Option<Ident> {
    let mut repr: Option<Ident> = None;

    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                repr = Some(ident.clone());
            }
            Ok(())
        });

        // Stop after first repr attribute
        if repr.is_some() {
            break;
        }
    }

    repr
}

#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    match input.data {
        Data::Struct(s) => {
            quote! {
                impl #impl_generics Serializable for #name #ty_generics #where_clause {
                    const SIZE: usize = 1;

                    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
                        // generated implementation
                    }
                }
            }
            .into()
        }
        Data::Enum(_) => {
            let repr = match enum_repr_type_name(&input.attrs) {
                None => {
                    return syn::Error::new_spanned(
                        input.ident,
                        "Serializable can only be derived on enums with explicit repr() attributes",
                    )
                        .to_compile_error()
                        .into()
                }
                Some(repr) => repr
            };

            quote! {
                impl #impl_generics Serializable for #name #ty_generics #where_clause {
                    const SIZE: usize = #repr::SIZE;

                    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
                        (*self as #repr).serialize_to(to, offset);
                    }
                }
            }
            .into()
        }
        Data::Union(_) => syn::Error::new_spanned(
            input.ident,
            "Serializable can only be derived for structs and enums",
        )
        .to_compile_error()
        .into(),
    }
}
