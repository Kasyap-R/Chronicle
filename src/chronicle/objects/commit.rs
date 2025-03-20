use crate::chronicle::{
    ignore::FilteredDirIter,
    paths,
    staging::index::{self},
};

use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use super::tree;

type PathHashes = HashMap<PathBuf, String>;

// Read the index, and generate the corresponding tree
// Then generate the commit which should just point to said tree
pub fn handle_commit(_message: String) -> Result<()> {
    let staged_file_hashes = index::get_staged_hashes()?;
    let prev_file_hashes = prev_file_hashes()?;
    let prev_tree_hashes = prev_tree_hashes()?;
    let changed_dirs =
        calc_changed_dirs(&staged_file_hashes, &prev_file_hashes, &prev_tree_hashes)?;

    let _commit_root_tree_hash = tree::get_tree_hash(
        Path::new(paths::ROOT_PATH),
        &changed_dirs,
        &prev_tree_hashes,
        &staged_file_hashes,
    );

    Ok(())
}

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
