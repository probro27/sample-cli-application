use std::io::{self, BufReader, Write};
use std::fs::File;
use std::io::prelude::*;

pub fn read_lines(filename: std::path::PathBuf) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename).expect("No file found");
    Ok(io::BufReader::new(file).lines())
}

pub fn find_matches(content: io::Lines<BufReader<File>>, pattern: String, mut writer: impl Write) {
    for line in content {
        if let Ok(line) = line {
            if line.contains(&pattern) {
                writeln!(writer, "{}", line).expect("Writer failed");
            }
        }
    }
}