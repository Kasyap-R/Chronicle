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

pub fn get_ignored_paths() -> Result<HashSet<PathBuf>> {
    let mut ignored_paths = HashSet::from([
        PathBuf::from(paths::CHRON_DIR_PATH).canonicalize()?,
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

// NOTE: Figure out how to make this generic ahh function
//pub fn traverse_valid_files<F, G>(dir_path: &Path, mut file_handler: F, mut dir_handler: G)
//where
//    F: FnMut(&Path),
//    G: FnMut(&Path),
//{
//    if let Ok(entries) = fs::read_dir(dir_path) {
//        for entry in entries.flatten() {
//            let path = entry.path();
//            if path.is_dir() {
//                dir_handler(&path); // Handle directory
//                traverse_valid_files(&path, &mut file_handler, &mut dir_handler); // Recurse
//            } else if path.is_file() {
//                file_handler(&path); // Handle file
//            }
//        }
//    }
//}
