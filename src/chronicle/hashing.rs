use anyhow::{Context, Result};
use sha1::{Digest, Sha1};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use super::paths;

fn sha1_hash(file_contents: &str) -> Result<String> {
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, file_contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn hash_file(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    hash_string(&file_contents).context(format!(
        "Failed to get hash for the following file: {}",
        file_path
            .to_str()
            .unwrap_or("Failed to retrieve file path.")
    ))
}

pub fn hash_string(string: &str) -> Result<String> {
    let hash = sha1_hash(&string)?;
    Ok(hash)
}

pub fn split_object_hash(hash: &str) -> (&str, &str) {
    assert!(hash.len() == 40);
    let directory_name = &hash[0..2];
    let file_name = &hash[2..];
    return (directory_name, file_name);
}

pub fn get_object_paths(hash: &str) -> (PathBuf, PathBuf) {
    let (directory_name, file_name) = split_object_hash(hash);

    let directory_path: PathBuf = [paths::OBJECTS_PATH, directory_name].into_iter().collect();
    let file_path: PathBuf = [directory_path.to_str().unwrap(), file_name]
        .into_iter()
        .collect();

    (directory_path, file_path)
}
