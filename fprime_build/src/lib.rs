use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

mod commands;
mod context;
mod telemetry;
mod types;
mod util;
mod value;

pub(crate) type Qualifier = Vec<Ident>;
struct CodeVec(Vec<(Qualifier, TokenStream)>);

struct CodeTree {
    leafs: Vec<TokenStream>,
    modules: BTreeMap<Ident, CodeTree>,
}

/// Consolidate definitions under the same module qualifier into a tree structure
impl From<CodeVec> for CodeTree {
    fn from(value: CodeVec) -> Self {
        let core_use = quote! {
            #[allow(unused_imports)]
            use fprime_core::*;
        };

        let mut root = CodeTree {
            leafs: vec![core_use.clone()],
            modules: Default::default(),
        };

        for (fq, def) in value.0 {
            let mut current_node = &mut root;

            // Traverse the tree from the root to the qualified point
            for q in fq {
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

            current_node.leafs.push(def);
        }

        root
    }
}

/// Serialize a tree of definitions into a stream of tokens
/// This works recursively
impl From<CodeTree> for TokenStream {
    fn from(value: CodeTree) -> Self {
        let modules: Vec<TokenStream> = value
            .modules
            .into_iter()
            .map(|(q, g)| {
                let inner: TokenStream = g.into();
                quote! {
                    pub mod #q {
                        #inner
                    }
                }
            })
            .collect();

        let leafs = value.leafs;
        quote! {
            #(#leafs)*
            #(#modules)*
        }
    }
}

/// Serialize code into formatted text
impl CodeTree {
    pub(crate) fn to_string(self) -> String {
        let ts: TokenStream = self.into();
        let s = ts.to_string();
        match syn::parse_file(&ts.to_string()) {
            Ok(parsed) => prettyplease::unparse(&parsed),
            Err(err) => format!("{}\n// {}\n", s, err.to_string()),
        }
    }
}

pub(crate) fn generate_to_file<W: ?Sized + Write>(
    dict: &fprime_dictionary::Dictionary,
    writer: &mut BufWriter<W>,
) {
    writer
        .write(
            Into::<CodeTree>::into(CodeVec(
                dict.type_definitions
                    .iter()
                    .map(types::type_definition)
                    .chain(dict.commands.iter().map(commands::command))
                    // .chain(
                    //     dict.telemetry_channels
                    //         .iter()
                    //         .map(telemetry::telemetry_channel),
                    // )
                    .collect(),
            ))
            .to_string()
            .as_bytes(),
        )
        .expect("failed to write to file");

    writer.flush().expect("failed to flush file")
}

pub fn generate(dictionary_json: &Path) {
    println!("cargo::rerun-if-changed=build.rs");

    // Get the output directory provided by Cargo
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.rs");

    let dict = fprime_dictionary::parse(dictionary_json);

    let file = fs::File::create(dest_path).expect("failed to open destination file");
    let mut writer = BufWriter::new(file);

    generate_to_file(&dict, &mut writer);
}

#[cfg(test)]
mod test {
    mod test;
}
