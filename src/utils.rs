use anyhow::Result;
use std::{
    fs::{self},
    path::Path,
};

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}
