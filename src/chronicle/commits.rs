use super::objects::ObjectType;
use super::objects::tree::create_tree_object;
use super::objects::{commit, tree::TreeEntry};
use crate::chronicle::staging::index::{self};
use tree_node::TreeNode;

use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

mod tree_node;

// NOTE: When committing, git doesn't walk the whole project directory, it walks only staged files (which
// are stored in index entries). Derive a tree structure from staged file paths only

// Read the index, and generate the corresponding tree
// Then generate the commit which should just point to said tree
pub fn handle_commit(message: String) -> Result<()> {
    let staged_file_hashes = index::get_staged_hashes()?;
    let commit_root = gen_tree_structure(&staged_file_hashes);
    let commit_root_tree_hash = gen_tree_object(&commit_root, &staged_file_hashes)?;
    commit::create_commit_object(commit_root_tree_hash, message)?;

    Ok(())
}

// Generate a tree structure that represents the staged files
fn gen_tree_structure(idx_entries: &HashMap<PathBuf, String>) -> TreeNode {
    let mut root = TreeNode::new();

    for (file_path, hash) in idx_entries.iter() {
        assert!(file_path.is_file());

        let mut components = file_path.components().peekable();
        let mut current = &mut root;

        while let Some(component) = components.next() {
            let component = component.as_os_str().to_str().unwrap().to_string();

            if components.peek().is_some() {
                current = current.get_or_add_dir(component);
            } else {
                // This is the file name
                current.add_file(component, hash.clone());
            }
        }
    }

    root
}

// Mark which trees are changed (by reading the previous commit)
// fn calc_changed_trees() {}

// Walk through the given tree structure and create the tree objects (using the file hashes that
// are staged). A future optimization is to track changed directories and only rebuild trees that
// have changed since last commit (and use the existing hash for ones that haven't)
fn gen_tree_object(node: &TreeNode, file_hashes: &HashMap<PathBuf, String>) -> Result<String> {
    let mut tree_entries: Vec<TreeEntry> = Vec::new();

    for (subdir_name, subdir_node) in &node.subdirs {
        let child_tree_entry = TreeEntry::new(
            ObjectType::Tree,
            subdir_name.clone(),
            gen_tree_object(&subdir_node, file_hashes)?,
        );

        tree_entries.push(child_tree_entry);
    }

    for (file_name, file_hash) in &node.files {
        let file_entry = TreeEntry::new(ObjectType::Blob, file_name.clone(), file_hash.clone());
        tree_entries.push(file_entry);
    }

    create_tree_object(tree_entries)
}
