mod cmd;

use std::path::PathBuf;

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
    Init {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        #[arg(short, long)]
        project_name: Option<String>,

        #[arg(short, long, default_value_t = false)]
        with_schemas: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Test) => {
            todo!("Test command logic goes here");
        }
        Some(Commands::Build) => {
            todo!("Build command logic goes here");
        }
        Some(Commands::Lint) => {
            todo!("Lint command logic goes here");
        }
        Some(Commands::Init {
            path,
            project_name,
            with_schemas,
        }) => {
            cmd::init::init_command(&path, project_name, with_schemas);
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
