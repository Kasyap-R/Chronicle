use anyhow::Result;
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

pub fn compress_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}
