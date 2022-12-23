use clap::Parser;
use grrs::{find_matches, read_lines};

#[derive(Parser)]
struct CLi {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = CLi::parse();
    
    // Slower implementation using Fs
    /*
        let file_content = fs::read_to_string(&args.path).expect("No file found");

        for line in file_content.lines() {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
     */

    // Faster implementation using BufReader
    if let Ok(lines) = read_lines(args.path) {
        find_matches(lines, args.pattern, &mut std::io::stdout());
    }
}
