use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use index::IndexEntry;

use crate::utils;

use super::objects::blob;
use super::{hashing, paths};

mod index;

// TODO: ignore files in .chronignore
// TODO: add support for git rm (which just removes files from the index)

pub fn handle_staging(path: &Path) -> Result<()> {
    let ignored_paths = utils::get_ignored_paths()?;
    if ignored_paths.contains(path) {
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
    println!("Staging files in directory: {}", dir_path.to_str().unwrap());
    let entries = fs::read_dir(dir_path)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if ignored.contains(&path) {
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
    let mut file = File::open(file_path)?;
    let metadata = fs::metadata(file_path)?;

    let file_name = file_path
        .to_str()
        .unwrap_or("Failed to retrieve file path.");
    let hash = hashing::get_file_hash(&mut file).context(format!(
        "Failed to get hash for file while staging: {}",
        file_name
    ))?;

    let file_size = utils::get_file_size(file_path)?;
    let last_modified = metadata.modified()?;

    add_index_entry(IndexEntry::new(
        file_path.to_path_buf(),
        hash,
        file_size,
        last_modified,
    ))?;

    blob::create_blob(file_path)?;

    Ok(())
}

fn add_index_entry(entry: IndexEntry) -> Result<()> {
    let mut index_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(paths::INDEX_PATH)?;

    let mut idx_file_contents = String::new();
    index_file.read_to_string(&mut idx_file_contents)?;

    let mut entries: Vec<IndexEntry> =
        serde_json::from_str(&idx_file_contents).unwrap_or_else(|_| Vec::new());

    update_index_entries(&mut entries, entry);

    let json_string = serde_json::to_string_pretty(&entries)?;
    index_file.set_len(0).context("Failed to clear file")?;
    index_file.seek(SeekFrom::Start(0))?; // <<<< Move cursor to start
    index_file.write_all(json_string.as_bytes())?;
    Ok(())
}

fn update_index_entries(entries: &mut Vec<IndexEntry>, entry: IndexEntry) {
    if let Some(idx) = entry.in_vec(entries) {
        entries[idx] = entry;
    } else {
        entries.push(entry);
    }
}
