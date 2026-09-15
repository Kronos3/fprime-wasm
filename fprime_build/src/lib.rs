use crate::tree::{CodeTree, Definition};
use fprime_dictionary::TypeDefinition;
use proc_macro2::TokenStream;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

mod commands;
mod telemetry;
mod tree;
mod types;
mod util;

/// Serialize a token stream into a string
fn render_tokens(ts: TokenStream) -> String {
    let s = ts.to_string();
    match syn::parse_file(&ts.to_string()) {
        Ok(parsed) => prettyplease::unparse(&parsed),
        Err(err) => format!("{}\n// {}\n", s, err.to_string()),
    }
}

pub(crate) fn generate_to_file<W: ?Sized + Write>(
    dict: &fprime_dictionary::Dictionary,
    writer: &mut BufWriter<W>,
) {
    let Some(TypeDefinition::Enum(cmd_response)) = &dict.type_definitions.get("Fw.CmdResponse")
    else {
        panic!("Fw.CmdResponse not found in dictionary");
    };

    let mut definitions = vec![];

    // Generate all namespace nested definitions
    for (_, ty) in &dict.type_definitions {
        let (qualifier, tokens) = types::type_definition(ty);
        definitions.push(Definition { qualifier, tokens });
    }

    // Generate all impl nested definitions
    let mut impls = vec![];

    for cmd in &dict.commands {
        let (qualifier, tokens) = commands::command(cmd, cmd_response);
        impls.push(Definition { qualifier, tokens });
    }

    for tlm in &dict.telemetry_channels {
        let (qualifier, tokens) = telemetry::telemetry_channel(tlm);
        impls.push(Definition { qualifier, tokens });
    }

    // Collect all the code into a hierarchy
    let definitions: CodeTree = definitions.into();
    let impls: CodeTree = impls.into();

    // Linearize the definition trees into a token stream nested in modules
    // Linearize the impl trees into a token stream of nested structs
    let tokens: TokenStream = definitions
        .module_nesting()
        .into_iter()
        .chain(impls.struct_nesting(dict))
        .collect();

    // Render the token stream into formatted Rust code and write it to a file
    writer
        .write(render_tokens(tokens).as_bytes())
        .expect("failed to write to file");

    writer.flush().expect("failed to flush file")
}

pub fn generate(dictionary_json: &str) {
    // Get the output directory provided by Cargo
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.rs");
    println!("cargo::rerun-if-changed=build.rs");
    // println!("cargo::rustc-flags=--remap-path-prefix={}=/b", out_dir.to_str().unwrap());

    let dict = fprime_dictionary::parse(Path::new(dictionary_json));

    let file = fs::File::create(dest_path).expect("failed to open destination file");
    let mut writer = BufWriter::new(file);

    generate_to_file(&dict, &mut writer);

    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed={}", dictionary_json);
}

#[cfg(test)]
mod test {
    mod test;
}
