use crate::chronicle::{hashing, paths};

use super::utils;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
    time::SystemTime,
};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct IndexEntry {
    pub last_modified: SystemTime,
    pub hash: String,
    pub file_size: u64,
}

impl IndexEntry {
    pub fn new(hash: String, file_size: u64, last_modified: SystemTime) -> Self {
        IndexEntry {
            hash,
            file_size,
            last_modified,
        }
    }

    pub fn create_index_entry(file_path: &Path, hash: &str) -> Result<IndexEntry> {
        let metadata = fs::metadata(file_path)?;
        let file_size = utils::get_file_size(file_path)?;
        let last_modified = metadata.modified()?;

        Ok(IndexEntry::new(hash.to_string(), file_size, last_modified))
    }
}

pub fn is_file_in_index(file_path: &Path, computed_hash: &mut Option<String>) -> Result<bool> {
    let idx_entries = get_index_file_entries();
    if !idx_entries.contains_key(file_path) {
        return Ok(false);
    }

    let entry = idx_entries.get(file_path).unwrap();
    let last_modified = utils::get_last_modified(file_path)?;
    if entry.last_modified == last_modified {
        return Ok(true);
    }

    // Its possible that even though the file was edited, the hash is the same (e.g. a change was
    // made then reverted)
    let hash = hashing::hash_file(file_path)?;
    *computed_hash = Some(hash.clone());
    Ok(hash == entry.hash)
}

pub fn update_index(entry_map: &HashMap<PathBuf, IndexEntry>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&entry_map)?;

    let mut index_file = OpenOptions::new().write(true).open(paths::INDEX_PATH)?;
    index_file.set_len(0).context("Failed to clear file")?;
    index_file.seek(SeekFrom::Start(0))?; // Move cursor to start
    index_file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn get_staged_hashes() -> Result<HashMap<PathBuf, String>> {
    let entries = get_index_file_entries().clone();
    let mut hashes = HashMap::new();
    entries.into_iter().for_each(|(file_path, entry)| {
        hashes.insert(file_path, entry.hash);
    });

    Ok(hashes)
}

fn calc_index_entries() -> Result<HashMap<PathBuf, IndexEntry>> {
    let mut idx_file = File::open(paths::INDEX_PATH)?;
    let mut idx_file_contents = String::new();
    idx_file.read_to_string(&mut idx_file_contents)?;
    let entry_map: HashMap<PathBuf, IndexEntry> =
        serde_json::from_str(&idx_file_contents).unwrap_or_else(|_| HashMap::new());
    Ok(entry_map)
}

static INDEX_ENTRIES: OnceLock<HashMap<PathBuf, IndexEntry>> = OnceLock::new();

pub fn get_index_file_entries() -> &'static HashMap<PathBuf, IndexEntry> {
    INDEX_ENTRIES.get_or_init(|| calc_index_entries().expect("Failed to read index file entries"))
}
