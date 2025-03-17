use anyhow::Result;
use sha1::{Digest, Sha1};
use std::{fs::File, io::Read};

fn sha1_hash(file_contents: &str) -> Result<String> {
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, file_contents);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub fn get_file_hash(file: &mut File) -> Result<String> {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let hash = sha1_hash(&file_contents)?;
    Ok(hash)
}
