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
    const OBJ_TYPE: ObjectType = ObjectType::Blob;

    fn read_obj_from(_obj_path: &Path) -> Self {
        Blob {
            contents: String::new(),
        }
    }

    fn obj_body(&self) -> String {
        self.contents.clone()
    }
}

pub fn create_blob(base_file_path: &Path) -> Result<String> {
    let base_file_contents = utils::read_raw_file(base_file_path)?;
    let blob = Blob::new(base_file_contents);
    blob.write_obj()
}
