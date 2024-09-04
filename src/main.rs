use std::process;
use log::{error, info};
use env_logger::Env;
use clap::Parser;
use artifact::Artifact;
use config::Config;

mod config;
mod artifact;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::parse();

    if let Err(e) = run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting file preparation process");
    info!("Source directory: {}", config.source_dir.display());
    info!("Destination directory: {}", config.dest_dir.display());
    info!("Ignored directories: {:?}", config.get_ignored_dirs());

    let artifacts = Artifact::collect(&config)?;

    Artifact::write_all(&artifacts, &config.dest_dir)?;

    info!("File preparation completed successfully");
    Ok(())
}