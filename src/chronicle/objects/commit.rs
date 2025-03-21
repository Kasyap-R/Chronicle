use crate::chronicle::prefix::Prefix;

use super::{ChronObject, ObjectType};
use anyhow::Result;
use std::path::Path;

struct Commit {
    tree_hash: String,
    message: String,
}

impl Commit {
    fn new(tree_hash: String, message: String) -> Self {
        Commit { tree_hash, message }
    }
}

impl ChronObject for Commit {
    fn read_obj_from(_obj_path: &Path) -> Self {
        Commit {
            tree_hash: String::new(),
            message: String::new(),
        }
    }

    fn to_obj_string(&self) -> String {
        let mut obj_body = String::from("\n");
        let tree_entry = String::from("tree ") + &self.tree_hash + "\n";
        obj_body.push_str(&tree_entry);

        let obj_len: u64 = obj_body.as_bytes().len().try_into().unwrap();
        let prefix = Prefix::new(ObjectType::Commit, obj_len).to_string();

        prefix + &obj_body
    }
}

fn create_commit_object(tree_hash: String, message: String) -> Result<String> {
    let commit = Commit::new(tree_hash, message);
    commit.write_obj()
}
