use anyhow::Result;
use rand::Rng;
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};

use crate::utils;

use super::paths;

pub fn hash_file(file_path: &Path) -> Result<String> {
    let file_contents = utils::read_raw_file(file_path)?;
    Ok(hash_string(&file_contents))
}

pub fn hash_string(string: &str) -> String {
    sha1_hash(&string)
}

pub fn split_object_hash_to_names(hash: &str) -> (&str, &str) {
    assert!(hash.len() == 40);
    let directory_name = &hash[0..2];
    let file_name = &hash[2..];
    return (directory_name, file_name);
}

pub fn split_object_hash_to_paths(hash: &str) -> (PathBuf, PathBuf) {
    let (directory_name, file_name) = split_object_hash_to_names(hash);

    let directory_path: PathBuf = [paths::OBJECTS_PATH, directory_name].into_iter().collect();
    let file_path: PathBuf = [directory_path.to_str().unwrap(), file_name]
        .into_iter()
        .collect();

    (directory_path, file_path)
}

pub fn gen_random_hash() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..20).map(|_| rng.r#gen()).collect();
    let mut hasher = Sha1::new();
    hasher.update(&random_bytes);
    hex::encode(hasher.finalize())
}

pub fn is_valid_hash(hash: &str) -> bool {
    hash.len() == 40 && hash.is_ascii()
}

fn sha1_hash(file_contents: &str) -> String {
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, file_contents);
    let result = hasher.finalize();
    hex::encode(result)
}
