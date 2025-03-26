use crate::chronicle::paths;
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

fn calc_ignored_paths() -> HashSet<PathBuf> {
    let mut ignored_paths = HashSet::new();

    // Always ignore these
    if let Ok(p) = PathBuf::from(paths::CHRON_DIR_PATH).canonicalize() {
        ignored_paths.insert(p);
    }
    if let Ok(p) = PathBuf::from(paths::IGNORE_PATH).canonicalize() {
        ignored_paths.insert(p);
    }

    // Read .ignore file
    let ignore_file = Path::new(paths::IGNORE_PATH);
    if !ignore_file.exists() {
        return ignored_paths;
    }

    if let Ok(mut file) = File::open(ignore_file) {
        let mut file_contents = String::new();
        if file.read_to_string(&mut file_contents).is_ok() {
            for line in file_contents.lines() {
                if is_invalid_ignore(line) {
                    continue;
                }
                let path = Path::new(line);
                if let Ok(p) = path.canonicalize() {
                    ignored_paths.insert(p);
                }
            }
        }
    }

    ignored_paths
}

fn is_invalid_ignore(line: &str) -> bool {
    let trimmed = line.trim();
    // Skip "." and ".."
    if trimmed == "." || trimmed == ".." {
        println!(
            "Warning: Ignoring `{}` in .chronignore. This will be skipped by chronicle",
            trimmed
        );
        return true;
    }
    false
}

use std::sync::OnceLock;
static IGNORED_FILES: OnceLock<HashSet<PathBuf>> = OnceLock::new();

pub fn get_ignored_paths() -> &'static HashSet<PathBuf> {
    IGNORED_FILES.get_or_init(|| calc_ignored_paths())
}
