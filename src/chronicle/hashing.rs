use anyhow::Result;
use sha1::{Digest, Sha1};
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn sha1_hash_file(file: &File) -> Result<String> {
    let mut hasher = Sha1::new();
    let mut buffer = String::new();
    let mut reader = BufReader::new(file);

    reader.read_to_string(&mut buffer)?;
    Digest::update(&mut hasher, buffer.as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result))
}
