use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "creb")]
#[command(about = "A minimal EPUB reader for the terminal")]
#[command(version = "0.1.0")]
pub struct CliArgs {
  /// EPUB file to open
  pub filename: PathBuf,

  /// Start at specific chapter (0-indexed)
  #[arg(short, long)]
  pub chapter: Option<usize>,

  /// Enable verbose output
  #[arg(short, long)]
  pub verbose: bool,
}
