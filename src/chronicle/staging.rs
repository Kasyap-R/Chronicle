use super::ignore;
use super::objects::blob;
use crate::chronicle::traversal::FilterUnignoredIter;
use crate::utils::{self};
use std::path::Path;

use anyhow::Result;
use index::IndexEntry;

pub mod index;

// TODO: add support for git rm (which just removes files from the index)

// TODO: Make this func less jank, there needs to be a cleaner way to avoid adding at all if the
// path the user passed is ignored
pub fn handle_staging(path: &Path) -> Result<()> {
    if ignore::get_ignored_paths().contains(&path.canonicalize()?) {
        return Ok(());
    }
    stage_files(&path)?;
    Ok(())
}

fn stage_files(curr_path: &Path) -> Result<()> {
    if curr_path.is_file() {
        stage_file(&curr_path)?;
        return Ok(());
    }

    println!(
        "Staging files in directory: {}",
        curr_path.to_str().unwrap()
    );

    let entries = FilterUnignoredIter::new(curr_path)?;
    for entry in entries {
        let new_path = entry?.path();
        stage_files(&new_path)?
    }

    Ok(())
}

fn stage_file(file_path: &Path) -> Result<()> {
    let mut entry_map = index::get_index_file_entries().clone();
    // TODO: This function doesn't work because it doesn't account for the fact that the blob hash
    // was made with the the prefix whereas the base file path won't have the prefix
    if index::is_file_in_index(file_path)? {
        return Ok(());
    }

    let blob_hash = blob::create_blob(file_path)?;
    let entry = IndexEntry::create_index_entry(file_path, &blob_hash)?;
    entry_map.insert(file_path.to_path_buf(), entry);

    index::update_index(&entry_map)?;

    Ok(())
}
