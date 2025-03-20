use std::{
    collections::{HashMap, HashSet},
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

use crate::utils::get_ignored_paths;

use super::{ChronObject, ObjectType};
use anyhow::{Result, bail};

struct Tree {
    prefix: String,
    entries: Vec<TreeEntry>,
}

impl ChronObject for Tree {
    fn to_obj_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}

struct TreeEntry {
    entry_type: ObjectType,
    name: String,
    hash: String,
}

// Recursively traverses directory and generates tree objects as neccesary
fn get_tree_hash(
    dir_path: &Path,
    changed_dirs: &HashSet<PathBuf>,
    prev_tree_hashes: &HashMap<PathBuf, String>,
    staged_file_hashes: &HashMap<PathBuf, String>,
) -> Result<Option<String>> {
    if !changed_dirs.contains(dir_path) {
        return Ok(Some(prev_tree_hashes.get(dir_path).unwrap().to_string()));
    }

    let tree_entries: Vec<TreeEntry> = Vec::new();
    let fs_entries = fs::read_dir(dir_path)?;
    for fs_entry in fs_entries {
        let fs_entry = fs_entry?;
        let path = fs_entry.path();
        if path.is_dir() {}
    }
    Ok(Some(create_tree_object()?))
}

pub fn create_tree_object() -> Result<String> {
    Ok(String::new())
}
