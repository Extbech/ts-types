use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{MANIFEST_FILE_NAME, TYPE_DIR};

pub fn write_manifest_entry(struct_name: &str, file_name: &str) {
    let manifest = TYPE_DIR.join(MANIFEST_FILE_NAME);
    let mut entries = BTreeMap::new();

    if manifest.exists() {
        if let Ok(contents) = fs::read_to_string(&manifest) {
            for line in contents.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    entries.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    entries.insert(struct_name.to_string(), file_name.to_string());
    let contents = entries
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("\n");

    if let Err(e) = fs::create_dir_all(TYPE_DIR.as_path()) {
        panic!("Failed to create dump directory: {}", e);
    }

    if let Err(e) = fs::write(&manifest, contents) {
        panic!("Failed to write manifest: {}", e);
    }
}

pub fn read_previous_output_path(struct_name: &str) -> Option<String> {
    let manifest = TYPE_DIR.join(MANIFEST_FILE_NAME);
    if !manifest.exists() {
        return None;
    }

    fs::read_to_string(&manifest)
        .ok()?
        .lines()
        .find_map(|line| {
            let (key, value) = line.split_once('=')?;
            (key == struct_name).then(|| value.to_string())
        })
}

pub fn write_ts_file(file_name: &str, content: &str, struct_name: &str) {
    if let Err(e) = fs::create_dir_all(TYPE_DIR.as_path()) {
        panic!("Failed to create dump directory: {}", e);
    }

    let full_path = TYPE_DIR.join(file_name);
    if let Some(parent) = full_path.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                panic!("Failed to create parent directory for output file: {}", e);
            }
        }
    }

    if let Some(previous_path) = read_previous_output_path(struct_name) {
        if previous_path != file_name {
            let previous_full_path = TYPE_DIR.join(&previous_path);
            if previous_full_path.exists() {
                if let Err(e) = fs::remove_file(previous_full_path) {
                    panic!("Failed to remove previous generated file: {}", e);
                }
            }
        }
    }

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&full_path)
    {
        Ok(file) => file,
        Err(e) => panic!("Failed to open or create file: {}", e),
    };

    if let Err(e) = writeln!(file, "{}", content) {
        panic!("Failed to write to file: {}", e);
    }

    write_manifest_entry(struct_name, file_name);
}
