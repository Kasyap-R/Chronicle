use crate::chronicle::{
    ignore::{FilteredDirIter, get_ignored_paths},
    objects::{ObjectType, tree, tree::TreeEntry},
    paths,
    staging::index::{self},
};

use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

// Read the index, and generate the corresponding tree
// Then generate the commit which should just point to said tree
pub fn handle_commit(_message: String) -> Result<()> {
    let staged_file_hashes = index::get_staged_hashes()?;
    let prev_file_hashes = prev_file_hashes()?;
    let prev_tree_hashes = prev_tree_hashes()?;
    let changed_dirs =
        calc_changed_dirs(&staged_file_hashes, &prev_file_hashes, &prev_tree_hashes)?;

    let _commit_root_tree_hash = get_tree_hash(
        Path::new(paths::ROOT_PATH),
        &changed_dirs,
        &prev_tree_hashes,
        &staged_file_hashes,
    );

    Ok(())
}

type PathHashes = HashMap<PathBuf, String>;

// Generates an map of file hashes by reading the most recent commit (stored in head)
fn prev_file_hashes() -> Result<PathHashes> {
    Ok(HashMap::new())
}

// Generates a map of tree hashes by reading the most recent commit (stored in head)
fn prev_tree_hashes() -> Result<PathHashes> {
    Ok(HashMap::new())
}

fn calc_changed_dirs(
    staged_file_hashes: &PathHashes,
    prev_file_hashes: &PathHashes,
    prev_tree_hashes: &PathHashes,
) -> Result<HashSet<PathBuf>> {
    let mut changed_dirs: HashSet<PathBuf> = HashSet::new();
    let _ = has_dir_changed(
        Path::new(paths::ROOT_PATH),
        staged_file_hashes,
        prev_file_hashes,
        prev_tree_hashes,
        &mut changed_dirs,
    )?;
    Ok(changed_dirs)
}

fn has_dir_changed(
    dir_path: &Path,
    staged_file_hashes: &PathHashes,
    prev_file_hashes: &PathHashes,
    prev_tree_hashes: &PathHashes,
    changed_dirs: &mut HashSet<PathBuf>,
) -> Result<bool> {
    if !prev_tree_hashes.contains_key(dir_path) {
        return mark_changed(changed_dirs, dir_path);
    }

    let fs_entries = FilteredDirIter::new(dir_path)?;
    for fs_entry in fs_entries {
        let path = fs_entry?.path();

        if path.is_dir()
            && has_dir_changed(
                &path,
                staged_file_hashes,
                prev_file_hashes,
                prev_tree_hashes,
                changed_dirs,
            )?
        {
            return mark_changed(changed_dirs, dir_path);
        }

        if path.is_file() && file_has_changed(&path, staged_file_hashes, prev_file_hashes) {
            return mark_changed(changed_dirs, dir_path);
        }
    }
    Ok(false)
}

// Helper function to insert into `changed_dirs` and return `Ok(true)`
fn mark_changed(changed_dirs: &mut HashSet<PathBuf>, path: &Path) -> Result<bool> {
    changed_dirs.insert(path.to_path_buf());
    Ok(true)
}

fn file_has_changed(
    path: &Path,
    staged_file_hashes: &PathHashes,
    prev_file_hashes: &PathHashes,
) -> bool {
    // Previously staged and committed but now unstaged
    if !staged_file_hashes.contains_key(path) && prev_file_hashes.contains_key(path) {
        return true;
    }

    // Previously unstaged but now staged
    if staged_file_hashes.contains_key(path) && !prev_file_hashes.contains_key(path) {
        return true;
    }

    // If file is both unstaged and was not previously commited, it is not tracked and should be
    // ignored
    if !staged_file_hashes.contains_key(path) && !prev_file_hashes.contains_key(path) {
        return false;
    }

    // If the file's hash in unchanged, return false
    staged_file_hashes.get(path).unwrap() != prev_file_hashes.get(path).unwrap()
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

    Ok(tree::create_tree_object(tree_entries)?)
}
