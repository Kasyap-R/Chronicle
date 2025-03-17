use std::{path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct IndexEntry {
    pub path: PathBuf,
    pub last_modified: SystemTime,
    pub hash: String,
    pub file_size: u64,
}

impl IndexEntry {
    pub fn new(path: PathBuf, hash: String, file_size: u64, last_modified: SystemTime) -> Self {
        IndexEntry {
            path,
            hash,
            file_size,
            last_modified,
        }
    }

    pub fn in_vec(&self, vec: &Vec<Self>) -> Option<usize> {
        for (idx, entry) in vec.iter().enumerate() {
            if entry.path == self.path {
                return Some(idx);
            }
        }
        return None;
    }
}
