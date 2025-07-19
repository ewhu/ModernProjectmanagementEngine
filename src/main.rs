// src/main.rs
/*
 * Main executable for ModernProjectmanagementEngine
 */

use clap::Parser;
use modernprojectmanagementengine::{Result, run};

#[derive(Parser)]
#[command(version, about = ModernProjectmanagementEngine - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
