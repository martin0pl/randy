use clap::{Parser, Subcommand};
use rand::Rng;

const DESCRIPTION: &str = "CLI tool to generate random things anywhere on your device\nMade by martin0pl\nGithub : https://github.com/martin0pl/randy";

#[derive(Parser)]
#[command(name = "randy")]
#[command(version)]
#[command(about = DESCRIPTION, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch a dice
    Dice {
        /// Number of faces of the dice
        #[arg(value_parser = clap::value_parser!(u32).range(1..),short, long, default_value_t = 6)]
        nb_faces: u32,
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Dice { nb_faces } => {
            println!("Launch a dice of {} faces...", nb_faces);

            let result = rng.gen_range(1..(nb_faces+1));

            println!("Result : {}", result);
        }
    }
}
