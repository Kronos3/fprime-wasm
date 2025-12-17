use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

mod commands;
mod context;
mod telemetry;
mod types;
mod util;
mod value;

pub(crate) fn generate_to_file<W: ?Sized + Write>(
    dict: &fprime_dictionary::Dictionary,
    writer: &mut BufWriter<W>,
) {
    for ty in &dict.type_definitions {
        let unformatted = types::type_definition(ty).to_string();
        match syn::parse_file(&unformatted) {
            Ok(parsed) => {
                writer
                    .write(prettyplease::unparse(&parsed).as_bytes())
                    .expect("failed to write to file");
            }
            Err(err) => {
                writer
                    .write(unformatted.as_bytes())
                    .expect("failed to write to file");
                writer
                    .write(format!("\n// {}", err.to_string()).as_bytes())
                    .expect("failed to write to file");
            }
        };

        writer
            .write("\n".as_bytes())
            .expect("failed to write to file");
    }

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
