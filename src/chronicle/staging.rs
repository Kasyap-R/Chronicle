use super::hashing;
use super::objects::blob;
use crate::utils::{self};
use std::{
    collections::HashSet,
    fs::{self},
    path::{Path, PathBuf},
};

use anyhow::Result;
use index::IndexEntry;

pub mod index;

// TODO: add support for git rm (which just removes files from the index)

// TODO: Stop normalizing and support a more freeform .chroignore where for example, target/ would
// ignore ANY paths with target/ in them

pub fn handle_staging(path: &Path) -> Result<()> {
    let ignored_paths = utils::get_ignored_paths()?;
    // Normalize paths before comparison to avoid situations like when ./target is viewed
    // differently than target/
    if ignored_paths.contains(&path.canonicalize()?) {
        return Ok(());
    }

    if fs::metadata(path)?.is_file() {
        stage_file(&path)?;
    } else {
        stage_directory(path, &ignored_paths)?;
    }

    Ok(())
}

fn stage_directory(dir_path: &Path, ignored: &HashSet<PathBuf>) -> Result<()> {
    let fs_entries = fs::read_dir(dir_path)?;
    for entry in fs_entries.flatten() {
        let path = entry.path();
        if ignored.contains(&path.canonicalize()?) {
            continue;
        }
        if path.is_dir() {
            stage_directory(&path, ignored)?;
        } else {
            stage_file(&path)?;
        }
    }
    Ok(())
}

fn stage_file(file_path: &Path) -> Result<()> {
    let mut entry_map = index::get_index_file_entries()?;
    let mut computed_hash = None;
    if index::is_file_in_index(&entry_map, file_path, &mut computed_hash)? {
        return Ok(());
    }
    // Ensure the hash is only ever computed once
    let computed_hash = computed_hash.unwrap_or(hashing::hash_file(file_path)?);

    let entry = IndexEntry::create_index_entry(file_path, &computed_hash)?;
    entry_map.insert(file_path.to_path_buf(), entry);
    index::update_index(&entry_map)?;

    blob::create_blob(file_path, &computed_hash)?;

    Ok(())
}
