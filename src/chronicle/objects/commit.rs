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
    const OBJ_TYPE: ObjectType = ObjectType::Commit;

    fn read_obj_from(_obj_path: &Path) -> Self {
        Commit {
            tree_hash: String::new(),
            message: String::new(),
        }
    }

    fn obj_body(&self) -> String {
        let mut obj_body = String::new();
        let tree_entry = String::from("tree ") + &self.tree_hash + "\n";
        obj_body.push_str(&tree_entry);
        obj_body.push_str(&self.message);
        obj_body
    }
}

fn create_commit_object(tree_hash: String, message: String) -> Result<String> {
    let commit = Commit::new(tree_hash, message);
    commit.write_obj()
}
