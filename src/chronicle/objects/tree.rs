use crate::chronicle::prefix::Prefix;

use super::{ChronObject, ObjectType};

use anyhow::Result;
use std::path::{Path, PathBuf};

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
        let mut obj_body = String::from("\n");
        for entry in &self.entries {
            obj_body.push_str(&entry.to_string());
        }

        let obj_len: u64 = obj_body.as_bytes().len().try_into().unwrap();
        let prefix = Prefix::new(ObjectType::Tree, obj_len).to_string();
        prefix + &obj_body
    }

    fn read_obj_from(_obj_path: &Path) -> Self {
        Tree {
            entries: Vec::new(),
        }
    }
}

pub struct TreeEntry {
    pub obj_type: ObjectType,
    pub obj_path: PathBuf,
    pub hash: String,
}

impl TreeEntry {
    pub fn new(obj_type: ObjectType, obj_path: PathBuf, hash: String) -> Self {
        TreeEntry {
            obj_type,
            obj_path,
            hash,
        }
    }

    fn to_string(&self) -> String {
        // Format is: `<obj_type> <path_to_base_file or dir>\0<obj_hash>`
        self.obj_type.to_string()
            + " "
            + &self.obj_path.to_string_lossy()
            + "\0"
            + &self.hash
            + "\n"
    }
}

pub fn create_tree_object(tree_entries: Vec<TreeEntry>) -> Result<String> {
    let tree = Tree::new(tree_entries);
    return tree.write_obj();
}
