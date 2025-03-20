use crate::chronicle::paths;

use anyhow::Result;
use std::{
    collections::HashSet,
    fs::{self, DirEntry, File, ReadDir},
    io::{self, Read},
    path::{Path, PathBuf},
};

pub struct FilteredDirIter<'a> {
    inner: ReadDir,
    ignored: &'a HashSet<PathBuf>,
}

impl<'a> FilteredDirIter<'a> {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(FilteredDirIter {
            inner: fs::read_dir(path)?,
            ignored: get_ignored_paths(),
        })
    }
}

// TODO: Stop canonicalizing and support a more freeform .chroignore where for example, target/ would
// ignore ANY paths with target/ in them
impl<'a> Iterator for FilteredDirIter<'a> {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            match entry {
                Ok(e) => {
                    let entry_path = e.path();
                    // Normalize paths before comparison to avoid situations like when ./target is viewed
                    // differently than target/
                    let canonical_path = match entry_path.canonicalize() {
                        Ok(p) => p,
                        Err(x) => return Some(Err(x)),
                    };
                    if !self.ignored.contains(&canonical_path) {
                        return Some(Ok(e));
                    }
                }
                Err(x) => return Some(Err(x)),
            }
        }
        None
    }
}

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
                let trimmed = line.trim();
                // Skip "." and ".."
                if trimmed == "." || trimmed == ".." {
                    eprintln!(
                        "Warning: Ignoring `{}` in .chronignore. These ignores are skipped by chronicle",
                        trimmed
                    );
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

use std::sync::OnceLock;
static CACHED_RESULT: OnceLock<HashSet<PathBuf>> = OnceLock::new();

pub fn get_ignored_paths() -> &'static HashSet<PathBuf> {
    CACHED_RESULT.get_or_init(|| calc_ignored_paths())
}
