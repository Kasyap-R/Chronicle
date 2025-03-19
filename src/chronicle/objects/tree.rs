use std::{
    collections::{HashMap, HashSet},
    fs::{self, read_dir}
    path::{Path, PathBuf},
};

use super::{ChronObject, ObjectType};
use anyhow::Result;

struct Tree {
    prefix: String,
    entries: Vec<TreeEntry>,
}

impl ChronObject for Tree {
    fn to_obj_bytes(&self) -> Vec<u8> {}
}

struct TreeEntry {
    entry_type: ObjectType,
    name: String,
    hash: String,
}

pub fn create_tree_object() -> Result<()> {
    Ok(())
}

// Recursively traverses directory and generates tree objects as neccesary
fn get_tree_hash(dir_path: &Path, changed_dirs: &HashSet<PathBuf>, prev_tree_hashes: &HashMap<PathBuf, String>) -> Result<String> {
    if 
    let fs_entries = fs::read_dir(dir_path);
    for fs_entry in fs_entries {
        let fs_entry = 
    }
}
