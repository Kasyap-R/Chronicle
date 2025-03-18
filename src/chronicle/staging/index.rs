use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
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
}
