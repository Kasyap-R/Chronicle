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

type PathHashes = HashMap<PathBuf, String>;

// Read the index, and generate the corresponding tree
// Then generate the commit which should just point to said tree
pub fn handle_commit(_message: String) -> Result<()> {
    // NOTE: when calculating file size (to generate prefix) do STRING.as_bytes().len() not
    // STRING.len()

    let staged_file_hashes = index::get_staged_hashes()?;
    let prev_file_hashes = prev_commit_file_hashes()?;
    let _prev_tree_hashes = prev_commit_tree_hashes()?;
    let _changed_dirs = calc_changed_dirs(&staged_file_hashes, &prev_file_hashes)?;
    Ok(())
}

// Generates an map of file hashes by reading the previous commit (stored in head)
fn prev_commit_file_hashes() -> Result<PathHashes> {
    Ok(HashMap::new())
}

// Generates a map of tree hashes by reading the previous commit (stored in head)
fn prev_commit_tree_hashes() -> Result<PathHashes> {
    Ok(HashMap::new())
}

fn calc_changed_dirs(staged: &PathHashes, last_commited: &PathHashes) -> Result<HashSet<PathBuf>> {
    let mut changed_dirs: HashSet<PathBuf> = HashSet::new();
    let _ = has_dir_changed(
        Path::new(paths::ROOT_PATH),
        staged,
        last_commited,
        &mut changed_dirs,
    )?;
    Ok(changed_dirs)
}

fn has_dir_changed(
    dir_path: &Path,
    staged: &PathHashes,
    last_commited: &PathHashes,
    changed_dirs: &mut HashSet<PathBuf>,
) -> Result<bool> {
    let fs_entries = FilteredDirIter::new(dir_path)?;
    for fs_entry in fs_entries {
        let fs_entry = fs_entry?;
        let path = fs_entry.path();
        if path.is_dir() {
            if has_dir_changed(&path, staged, last_commited, changed_dirs)? {
                return Ok(true);
            }
        }
        if path.is_file() {
            // Changed can mean a change in whether or not its tracked as well as a change in
            // contents
            if file_has_changed(&path, staged, last_commited) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn file_has_changed(path: &Path, staged: &PathHashes, last_commited: &PathHashes) -> bool {
    // Previously staged and committed but now unstaged
    if !staged.contains_key(path) && last_commited.contains_key(path) {
        return true;
    }

    // Previously unstaged but now staged
    if staged.contains_key(path) && !last_commited.contains_key(path) {
        return true;
    }

    // If file is both unstaged and was not previously commited, it is not tracked and should be
    // ignored
    if !staged.contains_key(path) && !last_commited.contains_key(path) {
        return false;
    }

    // If the file's hash in unchanged, return false
    staged.get(path).unwrap() != last_commited.get(path).unwrap()
}
