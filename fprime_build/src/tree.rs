use crate::util::{NameKind, format_name, str_to_ident};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub type Qualifier = Vec<String>;

pub struct Definition {
    pub qualifier: Qualifier,
    pub tokens: TokenStream,
}

pub struct CodeTree {
    leafs: Vec<TokenStream>,
    modules: BTreeMap<String, CodeTree>,
}

/// Consolidate definitions under the same module qualifier into a tree structure
impl From<Vec<Definition>> for CodeTree {
    fn from(value: Vec<Definition>) -> Self {
        let mut root = CodeTree {
            leafs: vec![],
            modules: Default::default(),
        };

        for def in value {
            let mut current_node = &mut root;

            // Traverse the tree from the root to the qualified point
            for q in def.qualifier {
                match current_node.modules.get_mut(&q) {
                    None => {
                        // Ensure we can traverse down the tree
                        current_node.modules.insert(
                            q.clone(),
                            CodeTree {
                                leafs: vec![],
                                modules: Default::default(),
                            },
                        );
                    }
                    Some(_) => {}
                }

                current_node = current_node.modules.get_mut(&q).unwrap()
            }

            current_node.leafs.push(def.tokens);
        }

        root
    }
}

impl CodeTree {
    pub fn module_nesting(self) -> TokenStream {
        let modules: Vec<TokenStream> = self
            .modules
            .into_iter()
            .map(|(q, g)| {
                let inner: TokenStream = g.module_nesting();
                let mod_name = str_to_ident(&format_name(NameKind::Module, &q));
                quote! {
                    pub mod #mod_name {
                        #[allow(unused_imports)]
                        use fprime_core::*;

                        #inner
                    }
                }
            })
            .collect();

        let leafs = self.leafs;
        quote! {
            #[allow(unused_imports)]
            use fprime_core::*;

            #(#leafs)*
            #(#modules)*
        }
    }

    fn struct_nesting_impl(self, scope: Qualifier) -> TokenStream {
        let members = self.modules.keys().cloned().collect::<Vec<_>>();

        let inner: Vec<_> = self
            .modules
            .into_iter()
            .map(|(next_ident, next_tree)| {
                let mut next_scope = scope.clone();
                next_scope.push(next_ident);

                next_tree.struct_nesting_impl(next_scope)
            })
            .collect();

        if scope.is_empty() {
            if !self.leafs.is_empty() {
                let leafs = self.leafs;
                let leafs = quote! { #(#leafs)* };
                assert!(false, "{}", leafs.to_string());
            }

            quote! {
                #(#inner)*
            }
        } else {
            // Build a private struct with `_` concatenated naming for holding the impls
            let name = str_to_ident(&scope.join("_").to_case(Case::Pascal));

            let member_defs = members.into_iter().map(|member_name| {
                let member_name = str_to_ident(&format_name(NameKind::StructMember, &member_name));
                let ty_name = str_to_ident(
                    &format!("{}_{}", scope.join("_"), member_name.to_string())
                        .to_case(Case::Pascal),
                );

                quote! {
                    #member_name: #ty_name,
                }
            });

            let leafs = self.leafs;
            quote! {
                #(#inner)*
                struct #name {
                    #(#member_defs)*
                }

                impl #name {
                    #(#leafs)*
                }
            }
        }
    }

    pub fn struct_nesting(self) -> TokenStream {
        self.struct_nesting_impl(vec![])
    }
}
