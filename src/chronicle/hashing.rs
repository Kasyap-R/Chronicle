use anyhow::{Context, Result};
use sha1::{Digest, Sha1};
use std::{fs::File, io::Read, path::Path};

fn sha1_hash(file_contents: &str) -> Result<String> {
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, file_contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn hash_file(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let hash = sha1_hash(&file_contents).context(format!(
        "Failed to get hash for the following file while staging: {}",
        file_path
            .to_str()
            .unwrap_or("Failed to retrieve file path.")
    ))?;
    Ok(hash)
}

pub fn hash_string(string: &str) -> Result<String> {
    let hash = sha1_hash(&string)?;
    Ok(hash)
}
