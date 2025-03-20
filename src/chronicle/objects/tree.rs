use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::chronicle::{
    compression, hashing,
    ignore::{FilteredDirIter, get_ignored_paths},
};

use super::{
    ChronObject, ObjectType, ensure_obj_dir_exists, generate_object_prefix, get_object_paths,
};
use anyhow::Result;

struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    fn new(entries: Vec<TreeEntry>) -> Self {
        Tree { entries }
    }
}

impl ChronObject for Tree {
    fn to_obj_string(&self) -> String {
        let mut tree_entries = String::from("\n");
        for entry in &self.entries {
            tree_entries.push_str(entry.obj_type.as_str());
            tree_entries.push(' ');
            tree_entries.push_str(&entry.obj_path.to_string_lossy());
            tree_entries.push('\0');
            tree_entries.push_str(entry.hash.as_str());
            tree_entries.push('\n');
        }
        let tree_len: u64 = tree_entries.as_bytes().len().try_into().unwrap();
        let mut tree_contents: String = generate_object_prefix(ObjectType::Tree, tree_len);
        tree_contents.push_str(&tree_entries);
        tree_contents
    }
}

struct TreeEntry {
    obj_type: ObjectType,
    obj_path: PathBuf,
    hash: String,
}

impl TreeEntry {
    fn new(obj_type: ObjectType, obj_path: PathBuf, hash: String) -> Self {
        TreeEntry {
            obj_type,
            obj_path,
            hash,
        }
    }
}

// Recursively traverses directory and generates tree objects as neccesary
pub fn get_tree_hash(
    dir_path: &Path,
    changed_dirs: &HashSet<PathBuf>,
    prev_tree_hashes: &HashMap<PathBuf, String>,
    staged_file_hashes: &HashMap<PathBuf, String>,
) -> Result<String> {
    assert!(!get_ignored_paths().contains(&dir_path.canonicalize()?));
    if !changed_dirs.contains(dir_path) {
        return Ok(prev_tree_hashes.get(dir_path).unwrap().to_string());
    }

    let mut tree_entries: Vec<TreeEntry> = Vec::new();
    let fs_entries = FilteredDirIter::new(dir_path)?;

    for fs_entry in fs_entries {
        let path = fs_entry?.path();
        if path.is_dir() {
            let subdir_hash =
                get_tree_hash(dir_path, changed_dirs, prev_tree_hashes, staged_file_hashes)?;
            tree_entries.push(TreeEntry::new(ObjectType::Tree, path, subdir_hash));
        } else if path.is_file() {
            if !staged_file_hashes.contains_key(&path) {
                continue; // This file hasn't been staged
            }
            let file_hash = staged_file_hashes.get(&path).unwrap().to_string();
            tree_entries.push(TreeEntry::new(ObjectType::Blob, path, file_hash));
        }
    }

    Ok(create_tree_object(tree_entries)?)
}

fn create_tree_object(tree_entries: Vec<TreeEntry>) -> Result<String> {
    let tree = Tree::new(tree_entries);
    let tree_string = tree.to_obj_string();
    let tree_hash = hashing::hash_string(&tree_string)?;
    let compressed_tree_contents = compression::compress_bytes(tree_string.as_bytes())?;
    write_tree_object(&tree_hash, compressed_tree_contents)?;
    Ok(tree_hash)
}

fn write_tree_object(hash: &str, compressed_tree_contents: Vec<u8>) -> Result<()> {
    ensure_obj_dir_exists(hash)?;
    let (_, obj_file_path) = get_object_paths(hash);
    let mut obj_file = File::create_new(obj_file_path)?;
    obj_file.write_all(&compressed_tree_contents)?;
    Ok(())
}
