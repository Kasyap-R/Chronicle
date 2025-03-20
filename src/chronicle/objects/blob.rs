use crate::chronicle::compression;

use super::*;
use anyhow::Result;
use std::{fs::File, io::Write, path::Path};

pub fn create_blob(base_file_path: &Path, hash: &str) -> Result<()> {
    ensure_obj_dir_exists(hash);
    let (_, obj_file_path) = get_object_paths(&hash);
    write_blob(base_file_path, &obj_file_path)?;
    Ok(())
}

fn write_blob(base_file_path: &Path, object_file_path: &Path) -> Result<()> {
    let mut object_file = File::create_new(object_file_path)?;
    let mut base_file = File::open(base_file_path)?;

    let prefix = generate_object_prefix(ObjectType::Blob, utils::get_file_size(base_file_path)?);
    let compressed_data = compression::compress_file(&mut base_file)?;

    object_file.write_all(&prefix.as_bytes())?;
    object_file.write_all(&compressed_data)?;

    Ok(())
}
