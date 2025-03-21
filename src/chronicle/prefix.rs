use anyhow::{Result, bail};

use super::objects::ObjectType;

pub struct Prefix {
    obj_type: ObjectType,
    obj_size: u64,
}

impl Prefix {
    pub fn new(obj_type: ObjectType, obj_size: u64) -> Self {
        Prefix { obj_type, obj_size }
    }

    pub fn to_string(&self) -> String {
        self.obj_type.to_string() + " " + &self.obj_size.to_string() + "\0\n"
    }
}

fn read_prefix(obj_file_contents: &str) -> Result<Prefix> {
    let (prefix_str, _) = obj_file_contents
        .split_once("\0")
        .expect("Expected null terminator in object file at end of prefix.");

    let prefix_contents: Vec<&str> = prefix_str.split_whitespace().collect();

    if prefix_contents.len() != 2 {
        bail!("Expected prefix to have 2 whitespace separated components, a type and size");
    }

    let obj_type = ObjectType::str_to_obj_type(prefix_contents[0]).unwrap();
    let obj_size: u64 = prefix_contents[1].parse()?;

    Ok(Prefix::new(obj_type, obj_size))
}
