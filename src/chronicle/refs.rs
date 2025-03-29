use std::{fs::File, io::Write, path::Path};

use anyhow::Result;

use crate::utils;

use super::paths;

pub fn update_refs(commit_hash: String) -> Result<()> {
    let branch_file_path = utils::read_raw_file(Path::new(paths::HEAD_PATH))?;
    let mut file = File::create(branch_file_path)?;
    file.write_all(commit_hash.as_bytes())?;
    Ok(())
}
