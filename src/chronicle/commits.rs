use super::objects::ObjectType;
use super::objects::tree::create_tree_object;
use super::objects::{commit, tree::TreeEntry};
use super::refs;
use crate::chronicle::staging::index::{self};
use tree_node::TreeNode;

use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

mod tree_node;

// NOTE: When committing, git doesn't walk the whole project directory, it walks only staged files (which
// are stored in the INDEX). Derive a tree structure from staged file paths only

// Read the index, and generate the corresponding tree
// Then generate the commit which should just point to said tree
pub fn handle_commit(message: String) -> Result<()> {
    let staged_file_hashes = index::get_staged_hashes()?;

    let commit_root = gen_tree_structure(&staged_file_hashes);
    let commit_root_tree_hash = gen_tree_object(&commit_root, &staged_file_hashes)?;
    let commit_hash = commit::create_commit_object(commit_root_tree_hash, message)?;

    refs::update_refs(commit_hash)?;
    Ok(())
}

// Generate a tree structure that represents the staged files
fn gen_tree_structure(staged_file_hashes: &HashMap<PathBuf, String>) -> TreeNode {
    let mut root = TreeNode::new();

    for (file_path, hash) in staged_file_hashes.iter() {
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

// A future optimization is to track changed directories and only rebuild trees that
// have changed since last commit (and use the existing hash for ones that haven't)
// Remaking existing trees is actually causing an error rn, since write_obj will return an error
// if the file already exists
fn gen_tree_object(node: &TreeNode, file_hashes: &HashMap<PathBuf, String>) -> Result<String> {
    let mut tree_entries: Vec<TreeEntry> = Vec::new();

    for (subdir_name, subdir_node) in &node.subdirs {
        let child_tree_entry = TreeEntry::new(
            ObjectType::Tree,
            subdir_name.clone(),
            gen_tree_object(&subdir_node, file_hashes)?,
        );
        println!("Committed files in directory: {}", subdir_name);
        tree_entries.push(child_tree_entry);
    }

    for (file_name, file_hash) in &node.files {
        let file_entry = TreeEntry::new(ObjectType::Blob, file_name.clone(), file_hash.clone());
        tree_entries.push(file_entry);
    }

    create_tree_object(tree_entries)
}

#[cfg(test)]
mod tests {

    use crate::chronicle::hashing;

    use super::*;

    #[test]
    fn simple_tree_gen() {
        let mut sample_file_hashes: HashMap<PathBuf, String> = HashMap::new();
        let rand_hash1 = hashing::gen_random_hash();
        let rand_hash2 = hashing::gen_random_hash();
        sample_file_hashes.insert(PathBuf::from("src/mod.rs"), rand_hash1.clone());
        sample_file_hashes.insert(PathBuf::from("utils/primary.rs"), rand_hash2.clone());

        let mut expected_tree_node = TreeNode::new();
        let src_dir = expected_tree_node.get_or_add_dir(String::from("src"));
        src_dir.add_file(String::from("mod.rs"), rand_hash1);

        let utils_dir = expected_tree_node.get_or_add_dir(String::from("utils"));
        utils_dir.add_file(String::from("primary.rs"), rand_hash2);

        assert_eq!(expected_tree_node, gen_tree_structure(&sample_file_hashes));
    }
}
