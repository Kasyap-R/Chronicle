use anyhow::Result;
use std::{
    fs::{self, File},
    io::Read,
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

pub fn read_file_from_path(path: &Path) -> Result<String> {
    let mut obj_file = File::open(path)?;
    let mut file_contents = String::new();
    obj_file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
