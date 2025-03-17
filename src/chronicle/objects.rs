use super::{hashing, paths};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn create_blob(file: &mut File) -> Result<()> {
    let hash = hashing::get_file_hash(file)?;

    if !blob_exists(&hash) {
        let (directory_path, file_path) = get_blob_paths(&hash);
        if !directory_path.exists() {
            fs::create_dir(directory_path)?;
        }
        write_blob(&hash, &file_path)?;
    }

    Ok(())
}

fn blob_exists(hash: &str) -> bool {
    let (_, file_path) = get_blob_paths(hash);
    file_path.exists() && file_path.is_file()
}

fn split_blob_hash(hash: &str) -> (&str, &str) {
    assert!(hash.len() == 40);
    let directory_name = &hash[0..2];
    let file_name = &hash[2..];
    return (directory_name, file_name);
}

fn get_blob_paths(hash: &str) -> (PathBuf, PathBuf) {
    let (directory_name, file_name) = split_blob_hash(hash);

    let directory_path: PathBuf = [paths::OBJECTS_PATH, directory_name].iter().collect();
    let file_path: PathBuf = [directory_path.to_str().unwrap(), file_name]
        .iter()
        .collect();

    (directory_path, file_path)
}

fn write_blob(hash: &str, file_path: &Path) -> Result<()> {
    let mut file = File::create_new(file_path)?;
    // You're supposed to compress the file with zlib and write corresponding bytes
    // Also we need the prefix first that ends with a null terminator
    file.write_all(hash.as_bytes())?;
    Ok(())
}
