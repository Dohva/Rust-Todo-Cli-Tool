use clap::{Parser, Subcommand};
use std::{fs, io::Write, process};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Echo { content: String },
    Add { content: String },
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
        Some(Commands::Add { content }) => {
            let file_result = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&source_file);

            let mut file: fs::File;
            match file_result {
                Ok(opened_file) => file = opened_file,
                Err(error) => {
                    eprintln!("Error loading source file. Error: {error}");
                    process::exit(1);
                }
            }
            // TODO pack into separate method to allow smoother error handling, eg. returning error results
            file.write(&content.as_bytes());
        }
        None => {}
    }
}
