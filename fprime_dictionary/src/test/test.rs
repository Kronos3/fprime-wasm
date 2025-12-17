use pretty_assertions::assert_eq;
use std::panic::panic_any;
use std::path::PathBuf;
use std::{env, fs};

pub(crate) fn run_test(file_path: &str) {
    // Compute the path to the FPP input and .ref.txt output
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/test");

    let mut json_file = path.clone();
    json_file.push(file_path);
    json_file.set_extension("json");

    let mut ref_file = path.clone();
    ref_file.push(file_path);
    ref_file.set_extension("ref.txt");

    let dict: crate::Dictionary = match serde_json::from_str(
        &fs::read_to_string(json_file.clone()).expect("failed to read json file"),
    ) {
        Ok(d) => d,
        Err(err) => panic_any(format!(
            "{}:{}:{} {}",
            json_file.to_str().unwrap(),
            err.line(),
            err.column(),
            err.to_string()
        )),
    };

    let output = format!("{:#?}", dict);

    // Validate the diagnostic messages against the reference file
    match env::var("FPRIME_UPDATE_REF") {
        Ok(_) => {
            // Update the ref file
            fs::write(ref_file, output).expect("failed to write ref.txt")
        }
        Err(_) => {
            // Read and compare against the ref file
            let ref_txt = fs::read_to_string(ref_file).expect("failed to read ref.txt");
            assert_eq!(ref_txt, output)
        }
    }
}

#[test] // Marks this function as a test
fn ref_topology() {
    run_test("RefTopologyDictionary")
}
