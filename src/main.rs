use ccwc::{count_bytes, count_chars, count_lines, count_words};
use clap::Parser;
use std::fs;

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

    /// File to process
    file: String,
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(&args.file).expect("Could not read file");
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

    result.push(args.file);

    println!("{}", result.join(" "));
}
