use crate::utils;

use super::{compression, hashing};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub mod blob;
pub mod commit;
pub mod tree;

pub enum ObjectType {
    Blob,
    Commit,
    Tree,
    Tag,
}

impl ObjectType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Blob => String::from("blob"),
            Self::Commit => String::from("commit"),
            Self::Tag => String::from("tag"),
            Self::Tree => String::from("tree"),
        }
    }

    pub fn str_to_obj_type(string: &str) -> Option<Self> {
        match string {
            "blob" => Some(Self::Blob),
            "commit" => Some(Self::Commit),
            "tag" => Some(Self::Tag),
            "tree" => Some(Self::Tree),
            _ => None,
        }
    }
}

trait ChronObject {
    fn read_obj_from(obj_path: &Path) -> Self;
    fn to_obj_string(&self) -> String;
    fn write_obj(&self) -> Result<String> {
        let obj_string = self.to_obj_string();
        let obj_hash = hashing::hash_string(&obj_string)?;
        let compressed_obj_contents = compression::compress_bytes(obj_string.as_bytes())?;

        ensure_obj_dir_exists(&obj_hash)?;

        let (_, obj_file_path) = hashing::get_object_paths(&obj_hash);
        let mut obj_file = File::create_new(obj_file_path)?;
        obj_file.write_all(&compressed_obj_contents)?;
        Ok(obj_hash)
    }
}

fn ensure_obj_dir_exists(hash: &str) -> Result<()> {
    let (directory_path, _) = hashing::get_object_paths(&hash);
    if !directory_path.exists() {
        fs::create_dir(directory_path)?;
    }
    Ok(())
}
