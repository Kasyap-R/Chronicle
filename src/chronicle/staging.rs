use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{Context, Result};
use index::IndexEntry;

use super::objects;
use super::{hashing, paths};

mod index;

// TODO: ignore files in .chronignore
// TODO: add support for git rm (which just removes files from the index)

pub fn handle_staging(path: &Path) -> Result<()> {
    if fs::metadata(path)?.is_file() {
        stage_file(&path)?;
    } else {
        stage_directory(path)?;
    }

    Ok(())
}

fn stage_directory(dir_path: &Path) -> Result<()> {
    let entries = fs::read_dir(dir_path)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            stage_directory(&path)?;
        } else {
            stage_file(&path)?;
        }
    }
    Ok(())
}

fn stage_file(file_path: &Path) -> Result<()> {
    let mut file = File::open(file_path)?;
    let metadata = fs::metadata(file_path)?;

    let hash = hashing::get_file_hash(&mut file)?;
    let file_size = metadata.len();
    let last_modified = metadata.modified()?;

    add_index_entry(IndexEntry::new(
        file_path.to_path_buf(),
        hash,
        file_size,
        last_modified,
    ))?;

    objects::create_blob(file_path)?;

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

    if !entries.contains(&entry) {
        entries.push(entry);
    }

    let json_string = serde_json::to_string_pretty(&entries)?;
    index_file.set_len(0).context("Failed to clear file")?;
    index_file.write_all(json_string.as_bytes())?;
    Ok(())
}
