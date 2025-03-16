use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;

pub fn init_chronicle_repo() -> Result<()> {
    fs::create_dir(".chronicle")?;

    init_head()?;
    init_config()?;
    init_refs()?;
    init_objects()?;

    Ok(())
}

fn init_config() -> Result<()> {
    File::create(".chronicle/config")?;
    Ok(())
}

fn init_refs() -> Result<()> {
    fs::create_dir(".chronicle/refs")?;
    fs::create_dir(".chronicle/refs/heads")?;
    fs::create_dir(".chronicle/refs/tags")?;
    Ok(())
}

fn init_head() -> Result<()> {
    let mut file = File::create(".chronicle/HEAD")?;
    file.write_all(b"ref: refs/heads/main")?;
    Ok(())
}

fn init_objects() -> Result<()> {
    fs::create_dir(".chronicle/objects")?;
    Ok(())
}
