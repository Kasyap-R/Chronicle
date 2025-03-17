use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;

use super::paths;

pub fn init_chronicle_repo() -> Result<()> {
    fs::create_dir(".chronicle")?;

    init_index()?;
    init_config()?;
    init_refs()?;
    init_objects()?;

    init_head()?;

    Ok(())
}

fn init_config() -> Result<()> {
    File::create(paths::CONFIG_PATH)?;
    Ok(())
}

fn init_refs() -> Result<()> {
    fs::create_dir(paths::REFS_PATH)?;
    fs::create_dir(paths::HEADS_PATH)?;
    fs::create_dir(paths::REMOTES_PATH)?;
    fs::create_dir(paths::TAGS_PATH)?;
    Ok(())
}

fn init_head() -> Result<()> {
    let mut file = File::create(paths::HEAD_PATH)?;
    file.write_all(b"ref: refs/heads/main")?;
    Ok(())
}

fn init_objects() -> Result<()> {
    fs::create_dir(paths::OBJECTS_PATH)?;
    Ok(())
}

fn init_index() -> Result<()> {
    File::create(paths::INDEX_PATH)?;
    Ok(())
}
