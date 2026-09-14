use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

pub type Qualifier = Vec<Ident>;

pub struct Definition {
    pub qualifier: Qualifier,
    pub tokens: TokenStream,
}

pub struct CodeTree {
    leafs: Vec<TokenStream>,
    modules: BTreeMap<Ident, CodeTree>,
}

/// Consolidate definitions under the same module qualifier into a tree structure
impl From<Vec<Definition>> for CodeTree {
    fn from(value: Vec<Definition>) -> Self {
        let core_use = quote! {
            #[allow(unused_imports)]
            use fprime_core::*;
        };

        let mut root = CodeTree {
            leafs: vec![core_use.clone()],
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
                                leafs: vec![core_use.clone()],
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
                quote! {
                    pub mod #q {
                        #inner
                    }
                }
            })
            .collect();

        let leafs = self.leafs;
        quote! {
            #(#leafs)*
            #(#modules)*
        }
    }

    pub fn struct_nesting(self) -> TokenStream {

    }
}
