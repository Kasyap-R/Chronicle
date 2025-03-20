use anyhow::Result;
use std::{
    fs::{self},
    path::Path,
    time::SystemTime,
};

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn get_last_modified(path: &Path) -> Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.modified()?)
}
