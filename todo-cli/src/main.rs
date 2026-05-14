use clap::{Parser, Subcommand};
use std::{fs, io::Write};

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

fn add(content: &str, source_file_path: &str) -> Result<String, String> {
    let file_result = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(source_file_path);

    let mut file: fs::File;
    match file_result {
        Ok(opened_file) => file = opened_file,
        Err(error) => {
            return Err(format!(
                "Source file could not be opened due to error: {error}"
            ));
        }
    }

    let content_to_write = format!("{content}\n");
    let content_bytes = content_to_write.as_bytes();

    match file.write(content_bytes) {
        Ok(bytes_written) => {
            if bytes_written < content_bytes.len() {
                return Err(String::from(
                    "Could not write full task bytes to source file",
                ));
            }
        }
        Err(_content) => return Err(String::from("Could not write to source file")),
    }
    Ok(format!("Successfully created task: \"{content}\""))
}

fn run(command: Option<Commands>, source_file_path: &str) -> Result<String, String> {
    match command {
        Some(Commands::Echo { content }) => Ok(content),
        Some(Commands::Add { content }) => add(&content, source_file_path),
        None => Err(String::from("Command not recognized")),
    }
}

fn main() {
    let source_file = "todos.txt";
    let cli = Cli::parse();

    match run(cli.command, source_file) {
        Ok(content) => println!("{content}"),
        Err(content) => eprintln!("Error prevented command completion: {content}"),
    }
}
