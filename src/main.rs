// src/main.rs
/*
 * Main executable for ArtifactsGenerator
 */

use clap::Parser;
use artifactsgenerator::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtifactsGenerator - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
