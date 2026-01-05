use ccwc::{count_bytes, count_chars, count_lines, count_words};
use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Count bytes
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,

    /// Count words
    #[arg(short = 'w', long)]
    words: bool,

    /// Count characters
    #[arg(short = 'm', long)]
    chars: bool,

    /// File to process (reads from stdin if not provided)
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let content = match &args.file {
        Some(path) => fs::read_to_string(path).expect("Could not read file"),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Could not read stdin");
            buffer
        }
    };

    let mut result: Vec<String> = Vec::new();

    if args.lines {
        result.push(count_lines(&content).to_string());
    }

    if args.words {
        result.push(count_words(&content).to_string());
    }

    if args.chars {
        result.push(count_chars(&content).to_string());
    }

    if args.bytes {
        result.push(count_bytes(&content).to_string());
    }

    if result.is_empty() {
        result.push(count_lines(&content).to_string());
        result.push(count_words(&content).to_string());
        result.push(count_bytes(&content).to_string());
    }

    if let Some(path) = args.file {
        result.push(path);
    }

    println!("{}", result.join(" "));
}
