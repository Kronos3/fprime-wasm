use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, Literal};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

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
            let size = s.fields.iter().map(|field| {
                let ty = &field.ty;
                quote! { <#ty as Serializable>::SIZE }
            });

            let serialize_to = s.fields.iter().enumerate().map(|(i, field)| match &field.ident {
                None => {
                    let name = Literal::usize_unsuffixed(i);
                    quote! { self.#name.serialize_to(to, offset); }
                },
                Some(name) => quote! { self.#name.serialize_to(to, offset); },
            });

            let deserialize_from = s.fields.iter().enumerate().map(|(i, field)| {
                let ty = &field.ty;
                let name = match &field.ident {
                    None => &Ident::new(
                        &format!("_{}", Literal::usize_unsuffixed(i)),
                        Span::call_site(),
                    ),
                    Some(name) => name,
                };

                quote! { let #name: #ty = Serializable::deserialize_from(from, offset); }
            });

            let field_names: Vec<Ident> = s
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| match &field.ident {
                    None => Ident::new(
                        &format!("_{}", Literal::usize_unsuffixed(i)),
                        Span::call_site(),
                    ),
                    Some(name) => name.clone(),
                })
                .collect();

            quote! {
                impl #impl_generics Serializable for #name #ty_generics #where_clause {
                    const SIZE: usize = 0 #(+ #size)*;

                    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
                        #(#serialize_to)*
                    }

                    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
                        #(#deserialize_from)*
                        Self {
                            #(#field_names,)*
                        }
                    }
                }
            }
            .into()
        }
        Data::Enum(e) => {
            let repr =
                match enum_repr_type_name(&input.attrs) {
                    None => return syn::Error::new_spanned(
                        input.ident,
                        "Serializable can only be derived on enums with explicit repr() attributes",
                    )
                    .to_compile_error()
                    .into(),
                    Some(repr) => repr,
                };

            let mut match_branches = vec![];
            for variant in &e.variants {
                match &variant.discriminant {
                    None => {
                        return syn::Error::new_spanned(
                            &variant.ident,
                            "Serializable can only be derived on enums with explicit values on all variants",
                        )
                            .to_compile_error()
                            .into()
                    }
                    Some((_, value)) => {
                        let name = &variant.ident;
                        match_branches.push(quote! {
                            #value => Self::#name,
                        })
                    }
                }
            }

            quote! {
                impl #impl_generics Serializable for #name #ty_generics #where_clause {
                    const SIZE: usize = #repr::SIZE;

                    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
                        (*self as #repr).serialize_to(to, offset);
                    }

                    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
                        let raw: #repr = Serializable::deserialize_from(from, offset);
                        match raw {
                            #(#match_branches)*
                            _ => panic!("invalid value: {}", raw),
                        }
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
