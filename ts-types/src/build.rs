use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // This is where we collect the TypeScript definitions stored by the macro
    let ts_type_definitions = collect_ts_definitions();

    // Define the output path for the TypeScript file
    let out_dir = env::var("OUT_DIR").unwrap();
    let ts_file_path = Path::new(&out_dir).join("generated_types.ts");

    // Write the collected TypeScript definitions to the file
    let mut file = File::create(&ts_file_path).expect("Could not create TypeScript file");
    for ts_type in ts_type_definitions {
        writeln!(file, "{}", ts_type).expect("Failed to write to TypeScript file");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

// Function to collect the TypeScript definitions from the global storage
fn collect_ts_definitions() -> Vec<String> {
    let ts_definitions = crate_name::TS_TYPE_DEFINITIONS.lock().unwrap();
    ts_definitions.clone()
}
