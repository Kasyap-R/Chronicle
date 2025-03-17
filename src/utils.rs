use crate::chronicle::paths;
use anyhow::Result;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn get_ignored_paths() -> Result<HashSet<PathBuf>> {
    let mut ignored_paths = HashSet::from([
        PathBuf::from(paths::CHRON_DIR).canonicalize()?,
        PathBuf::from(paths::IGNORE_PATH).canonicalize()?,
    ]);
    let mut file = File::open(paths::IGNORE_PATH)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    for line in file_contents.split("\n") {
        let path = Path::new(line);
        if !path.exists() {
            continue;
        }
        ignored_paths.insert(path.canonicalize()?.to_path_buf());
    }

    Ok(ignored_paths)
}
