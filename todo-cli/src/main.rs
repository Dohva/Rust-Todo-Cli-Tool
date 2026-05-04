use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Echo {
        #[arg(short, long)]
        content: String,
    },
}

fn main() {
    let source_file = "todos.txt";
    let cli = Cli::parse();

    println!("checking if source file exists");
    match fs::File::create_new(source_file) {
        Ok(_file) => println!("File not found. Successfully created"),
        Err(_error) => println!("File already exists"),
    };

    match &cli.command {
        Some(Commands::Echo { content }) => {
            println!("{content}");
        }
        None => {}
    }
}
