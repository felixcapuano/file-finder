use std::{fs, path::PathBuf};

fn main() {
    let search_str: &str = "main.rs";

    let mut path: PathBuf = PathBuf::new();
    path.push("../");

    find(path, search_str);
}

fn find(path: PathBuf, search_str: &str) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        let file_name_str = path.file_name().unwrap().to_str().unwrap();
        if file_name_str.contains(search_str) {
            println!("{}", path.display());
        }
        if path.is_dir() {
            find(path, search_str)
        }
    }
}
