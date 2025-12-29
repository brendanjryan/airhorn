mod audio;
mod player;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "airhorn")]
#[command(about = "The airhorn soundclip you know and love, in your terminal", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Suppress stderr output unless verbose mode is enabled
    if cli.verbose {
        player::play_airhorn()
    } else {
        // Temporarily redirect stderr to null
        let _handle = gag::Gag::stderr().ok();
        player::play_airhorn()
    }
}
