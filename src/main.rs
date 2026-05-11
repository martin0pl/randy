use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "randy")]
#[command(about = "CLI tool to generate random things anywhere on your device", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch a dice
    Dice {
        /// Number of faces of the dice
        nb_faces: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dice { nb_faces } => {
            println!("Launch a dice of {} faces", nb_faces);
        }
    }
}
