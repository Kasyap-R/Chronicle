use anyhow::Result;

mod args;
mod chronicle;
mod utils;

fn main() -> Result<()> {
    chronicle::process_command()?;
    Ok(())
}
