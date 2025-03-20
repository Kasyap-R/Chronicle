use crate::chronicle::paths;

use anyhow::Result;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn get_last_modified(path: &Path) -> Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.modified()?)
}

fn calc_ignored_paths() -> HashSet<PathBuf> {
    if !Path::new(paths::IGNORE_PATH).exists() {
        return HashSet::new();
    }
    let mut ignored_paths = HashSet::from([
        PathBuf::from(paths::CHRON_DIR_PATH).canonicalize().unwrap(),
        PathBuf::from(paths::IGNORE_PATH).canonicalize().unwrap(),
    ]);

    let mut file = File::open(paths::IGNORE_PATH).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    for line in file_contents.split("\n") {
        let path = Path::new(line);
        if !path.exists() {
            continue;
        }
        ignored_paths.insert(path.canonicalize().unwrap().to_path_buf());
    }

    ignored_paths
}

use std::sync::OnceLock;
static CACHED_RESULT: OnceLock<Result<HashSet<PathBuf>>> = OnceLock::new();

pub fn get_ignored_paths() -> Result<&'static HashSet<PathBuf>> {
    CACHED_RESULT
        .get_or_init(|| Ok(calc_ignored_paths()))
        .as_ref()
        .map_err(|e| anyhow::anyhow!(e.to_string())) // Convert &Error to Error
}
