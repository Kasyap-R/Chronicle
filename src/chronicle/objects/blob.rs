use crate::chronicle::compression;

use super::hashing;
use super::*;
use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn create_blob(base_file_path: &Path) -> Result<()> {
    let mut base_file = File::open(base_file_path)?;
    let hash = hashing::get_file_hash(&mut base_file)?;

    if !object_exists(&hash) {
        let (directory_path, file_path) = get_object_paths(&hash);
        if !directory_path.exists() {
            fs::create_dir(directory_path)?;
        }
        write_blob(base_file_path, &file_path)?;
    }

    Ok(())
}

fn write_blob(base_file_path: &Path, object_file_path: &Path) -> Result<()> {
    let mut object_file = File::create_new(object_file_path)?;
    let mut base_file = File::open(base_file_path)?;
    let prefix = generate_object_prefix(&ObjectType::Blob, utils::get_file_size(base_file_path)?);
    let compressed_data = compression::compress_file(&mut base_file)?;
    object_file.write_all(&prefix)?;
    object_file.write_all(&compressed_data)?;
    Ok(())
}
