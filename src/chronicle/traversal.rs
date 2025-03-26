use crate::chronicle::ignore;
use anyhow::Result;
use std::{
    collections::HashSet,
    fs::{self, DirEntry, ReadDir},
    io::{self},
    path::{Path, PathBuf},
};

pub struct FilterUnignoredIter<'a> {
    inner: ReadDir,
    ignored: &'a HashSet<PathBuf>,
}

impl<'a> FilterUnignoredIter<'a> {
    pub fn new(path: &Path) -> Result<Self> {
        assert!(path.is_dir());

        Ok(FilterUnignoredIter {
            inner: fs::read_dir(path)?,
            ignored: ignore::get_ignored_paths(),
        })
    }
}

// TODO: Stop canonicalizing and support a more freeform .chroignore where for example, target/ would
// ignore ANY paths with target/ in them
impl<'a> Iterator for FilterUnignoredIter<'a> {
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
