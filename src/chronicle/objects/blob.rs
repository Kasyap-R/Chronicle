use crate::chronicle::prefix::Prefix;

use super::*;
use anyhow::Result;
use std::path::Path;

struct Blob {
    contents: String,
}

impl Blob {
    fn new(contents: String) -> Self {
        Blob { contents }
    }
}

impl ChronObject for Blob {
    fn read_obj_from(_obj_path: &Path) -> Self {
        Blob {
            contents: String::new(),
        }
    }

    fn to_obj_string(&self) -> String {
        let obj_body = &self.contents;

        let obj_len: u64 = self.contents.as_bytes().len().try_into().unwrap();
        let prefix = Prefix::new(ObjectType::Blob, obj_len).to_string();

        prefix + obj_body
    }
}

pub fn create_blob(base_file_path: &Path) -> Result<String> {
    let base_file_contents = utils::read_file_from_path(base_file_path)?;
    let blob = Blob::new(base_file_contents);
    return blob.write_obj();
}
