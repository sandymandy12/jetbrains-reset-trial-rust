mod cli;
mod core;
mod integrations;

use anyhow::Result;

fn main() -> Result<()> {
    // Run CLI by default
    cli::run_cli()?;

    Ok(())
}
