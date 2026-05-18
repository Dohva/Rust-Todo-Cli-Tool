#![warn(clippy::pedantic)]
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, BufRead, Write};

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
    Ls {},
    Do { index: usize },
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

fn list(source_file_path: &str) -> Result<String, String> {
    let file = match fs::OpenOptions::new().read(true).open(source_file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not open source file: {e}")),
    };

    let result = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(i, l)| match l {
            Ok(line) => format!("[{}] {}", i + 1, line),
            Err(e) => format!("[{}] ERROR reading line: {}", i, e),
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(result)
}

fn complete_task(index: usize, source_file_path: &str) -> Result<String, String> {
    let file = match fs::OpenOptions::new().read(true).open(source_file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not open source file: {e}")),
    };

    let lines = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(i, l)| match l {
            Ok(line) => (i + 1, line),
            Err(e) => (i, format!("ERROR reading line: {}", e)),
        })
        .collect::<Vec<(usize, String)>>();

    let remaining_lines = lines
        .iter()
        .filter(|x| x.0 != index)
        .map(|x| x.1.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    let mut file = match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(source_file_path)
    {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not write to source file: {e}")),
    };

    let bytes = remaining_lines.as_bytes();

    match file.write_all(bytes) {
        Ok(_) => Ok(format!("Completed item [{index}]")),
        Err(error) => Err(format!("Error occured trying to write to file: {error}")),
    }
}

fn run(command: Option<Commands>, source_file_path: &str) -> Result<String, String> {
    match command {
        Some(Commands::Echo { content }) => Ok(content),
        Some(Commands::Add { content }) => add(&content, source_file_path),
        Some(Commands::Ls {}) => list(source_file_path),
        Some(Commands::Do { index }) => complete_task(index, source_file_path),
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
