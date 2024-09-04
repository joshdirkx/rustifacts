use std::io::{self, Write};
use clap::Parser;
use artifact::Artifact;
use config::Config;

mod config;
mod artifact;

fn main() -> io::Result<()> {
    let config = Config::parse();
    let artifacts = Artifact::collect(&config)?;

    Artifact::write_all(&artifacts, &config.dest_dir)?;
    Artifact::generate_summary(&artifacts, &config.dest_dir)?;

    Ok(())
}