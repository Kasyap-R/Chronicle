use super::{hashing, paths};
use anyhow::Result;
use flate2::{Compression, write::ZlibEncoder};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn create_blob(base_file_path: &Path) -> Result<()> {
    let mut base_file = File::open(base_file_path)?;
    let hash = hashing::get_file_hash(&mut base_file)?;

    if !blob_exists(&hash) {
        let (directory_path, file_path) = get_blob_paths(&hash);
        if !directory_path.exists() {
            fs::create_dir(directory_path)?;
        }
        write_blob(base_file_path, &file_path)?;
    }

    Ok(())
}

fn blob_exists(hash: &str) -> bool {
    let (_, file_path) = get_blob_paths(hash);
    file_path.exists() && file_path.is_file()
}

fn split_blob_hash(hash: &str) -> (&str, &str) {
    assert!(hash.len() == 40);
    let directory_name = &hash[0..2];
    let file_name = &hash[2..];
    return (directory_name, file_name);
}

fn get_blob_paths(hash: &str) -> (PathBuf, PathBuf) {
    let (directory_name, file_name) = split_blob_hash(hash);

    let directory_path: PathBuf = [paths::OBJECTS_PATH, directory_name].iter().collect();
    let file_path: PathBuf = [directory_path.to_str().unwrap(), file_name]
        .iter()
        .collect();

    (directory_path, file_path)
}

fn write_blob(base_file_path: &Path, object_file_path: &Path) -> Result<()> {
    let mut object_file = File::create_new(object_file_path)?;
    let mut base_file = File::open(base_file_path)?;
    let prefix = generate_blob_prefix(base_file_path)?;
    let compressed_data = compress_file(&mut base_file)?;
    object_file.write_all(&prefix)?;
    object_file.write_all(&compressed_data)?;
    Ok(())
}

fn generate_blob_prefix(base_file_path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::metadata(base_file_path)?;
    let num_bytes = metadata.len();
    let mut prefix = Vec::new();
    prefix.extend_from_slice(b"blob ");
    prefix.extend_from_slice(num_bytes.to_string().as_bytes());
    prefix.push(0); // null terminator
    Ok(prefix)
}

fn compress_file(file: &mut File) -> Result<Vec<u8>> {
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&file_contents)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}
