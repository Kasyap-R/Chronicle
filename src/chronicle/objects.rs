use crate::utils;

use super::paths;
use std::path::PathBuf;

pub mod blob;
pub mod commit;
pub mod tree;

enum ObjectType {
    Blob,
    Commit,
    Tree,
    Tag,
}

impl ObjectType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Blob => "blob",
            Self::Commit => "commit",
            Self::Tag => "tag",
            Self::Tree => "tree",
        }
    }
}

trait ChronObject {
    fn to_obj_bytes(&self) -> Vec<u8>;
}

fn object_exists(hash: &str) -> bool {
    let (_, file_path) = get_object_paths(hash);
    file_path.exists() && file_path.is_file()
}

fn split_object_hash(hash: &str) -> (&str, &str) {
    assert!(hash.len() == 40);
    let directory_name = &hash[0..2];
    let file_name = &hash[2..];
    return (directory_name, file_name);
}

fn get_object_paths(hash: &str) -> (PathBuf, PathBuf) {
    let (directory_name, file_name) = split_object_hash(hash);

    let directory_path: PathBuf = [paths::OBJECTS_PATH, directory_name].iter().collect();
    let file_path: PathBuf = [directory_path.to_str().unwrap(), file_name]
        .iter()
        .collect();

    (directory_path, file_path)
}

fn generate_object_prefix(obj_type: &ObjectType, base_file_size: u64) -> Vec<u8> {
    let mut prefix = Vec::new();
    prefix.extend_from_slice(obj_type.as_str().as_bytes());
    prefix.extend_from_slice(base_file_size.to_string().as_bytes());
    prefix.push(0); // null terminator
    prefix
}
