use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut out_path = manifest_dir.clone();
    let profile = env::var("PROFILE").unwrap();
    out_path.push(format!("target/{}", profile));

    let mut resource_path = manifest_dir;
    resource_path.push("assets");
    let mut resource_out_path = out_path;
    resource_out_path.push("assets");
    copy_dir_content_recursive(resource_path.as_path(), &resource_out_path);
}

fn copy_dir_content_recursive(dir: &Path, destination_dir: &PathBuf) {
    if dir.is_dir() {
        if !destination_dir.as_path().exists() {
            fs::create_dir(destination_dir.as_path());
        }

        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut new_destination = destination_dir.clone();
                new_destination.push(entry.file_name());
                copy_dir_content_recursive(&path, &new_destination);
            } else {
                let mut new_file_path = destination_dir.clone();
                new_file_path.push(entry.file_name());
                std::fs::copy(&path, new_file_path.as_path())
                    .expect("Can't copy from Resource dir");
            }
        }
    }
}
