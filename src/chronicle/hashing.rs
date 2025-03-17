use anyhow::Result;
use sha1::{Digest, Sha1};

pub fn sha1_hash(file_contents: &String) -> Result<String> {
    let mut hasher = Sha1::new();
    Digest::update(&mut hasher, file_contents);
    let result = hasher.finalize();

    Ok(hex::encode(result))
}
