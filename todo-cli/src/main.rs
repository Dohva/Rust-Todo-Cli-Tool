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

fn run(command: Option<Commands>, source_file_path: &str) -> Result<String, String> {
    match command {
        Some(Commands::Echo { content }) => return Ok(content),
        Some(Commands::Add { content }) => {
            let file_result = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&source_file_path);

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
            return Ok(format!("Successfully created task: \"{content}\""));
        }
        None => return Err(String::from("Command not recognized")),
    };
}

fn main() {
    let source_file = "todos.txt";
    let cli = Cli::parse();

    println!("checking if source file exists");
    match fs::File::create_new(source_file) {
        Ok(_file) => println!("File not found. Successfully created"),
        Err(_error) => println!("File already exists"),
    };

    match run(cli.command, source_file) {
        Ok(content) => println!("{content}"),
        Err(content) => eprintln!("Error prevented command completion: {content}"),
    }
}
