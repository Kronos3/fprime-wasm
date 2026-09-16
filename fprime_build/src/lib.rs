use crate::tree::{CodeTree, Definition};
use fprime_dictionary::TypeDefinition;
use proc_macro2::TokenStream;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

mod commands;
mod constants;
mod parameters;
mod telemetry;
mod tree;
mod types;
mod util;
mod values;

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
    // Every command reports an `Fw::CmdResponse`
    if !matches!(
        dict.type_definitions.get("Fw.CmdResponse"),
        Some(TypeDefinition::Enum(_))
    ) {
        panic!("Fw.CmdResponse enum not found in dictionary");
    }

    let mut definitions = vec![];

    // Generate all namespace nested definitions
    // Definitions are keyed by an unordered map, so walk them by name to keep
    // the generated file stable across runs
    let mut type_names: Vec<&String> = dict.type_definitions.keys().collect();
    type_names.sort();

    for name in type_names {
        let (qualifier, tokens) = types::type_definition(&dict.type_definitions[name]);
        definitions.push(Definition { qualifier, tokens });
    }

    for c in &dict.constants {
        let (qualifier, tokens) = constants::constant(c, &dict);
        definitions.push(Definition { qualifier, tokens });
    }

    // Generate all impl nested definitions
    let mut impls = vec![];

    for cmd in &dict.commands {
        let (qualifier, tokens) = commands::command(cmd);
        impls.push(Definition { qualifier, tokens });
    }

    for tlm in &dict.telemetry_channels {
        let (qualifier, tokens) = telemetry::telemetry_channel(tlm);
        impls.push(Definition { qualifier, tokens });
    }

    for prm in &dict.parameters {
        let (qualifier, tokens) = parameters::parameter(prm);
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

    let dictionary_path = fs::canonicalize(dictionary_json).unwrap_or_else(|err| {
        panic!(
            "failed to resolve dictionary '{}': {}",
            dictionary_json, err
        )
    });

    let dict = fprime_dictionary::parse(&dictionary_path);

    let file = fs::File::create(dest_path).expect("failed to open destination file");
    let mut writer = BufWriter::new(file);

    generate_to_file(&dict, &mut writer);

    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed={}", dictionary_json);
    println!(
        "cargo::rustc-env={}={}",
        fprime_dictionary::DICTIONARY_ENV,
        dictionary_path.display()
    );
}

#[cfg(test)]
mod test {
    mod test;
}
