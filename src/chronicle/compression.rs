use anyhow::Result;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn compress_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub fn read_compressed_file(file_path: &Path) -> Result<String> {
    let file = File::open(file_path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    let string = String::from_utf8(decompressed_data)?;
    Ok(string)
}
