use anyhow::{Result, bail};

use super::{
    compression::read_compressed_file,
    hashing::{self, is_valid_hash},
};

pub fn print_obj_file(hash: String) -> Result<()> {
    if !is_valid_hash(&hash) {
        bail!("Provided hash is invalid.");
    }

    let (_, file_path) = hashing::split_object_hash_to_paths(&hash);

    if !file_path.exists() {
        bail!("Provided hash does not point to an existing object file.");
    }

    let decompressed_file_contents = read_compressed_file(&file_path)?;
    println!("{}", decompressed_file_contents);
    Ok(())
}
