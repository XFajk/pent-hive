mod cmd;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "MyApp", version = "1.0", author = "Rudolf Vrbensky")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test,
    Build,
    Lint,
    Init,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Test) => {
            todo!("Test command logic goes here");
        }
        Some(Commands::Build) => {
            todo!("Build command logic goes here");
        }
        Some(Commands::Lint) => {
            todo!("Lint command logic goes here");
        }
        Some(Commands::Init) => {
            cmd::init::init_command();
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
