use std::fs;
use clap::Parser;

#[derive(Parser)]
struct CLi {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = CLi::parse();
    let file_content = fs::read_to_string(&args.path).expect("No file found");

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
