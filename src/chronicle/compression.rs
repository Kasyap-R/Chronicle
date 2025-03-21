use anyhow::Result;
use flate2::{Compression, write::ZlibEncoder};
use std::fs::File;
use std::io::{Read, Write};

pub fn compress_file(file: &mut File) -> Result<Vec<u8>> {
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&file_contents)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}
