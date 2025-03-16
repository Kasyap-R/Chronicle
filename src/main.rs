use anyhow::Result;

mod args;
mod chronicle;

fn main() -> Result<()> {
    chronicle::process_command()?;
    Ok(())
}
