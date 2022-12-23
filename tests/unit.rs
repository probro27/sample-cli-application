use std::io::prelude::*;
use std::fs::File;
use grrs::{find_matches, read_lines};

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let mut file = File::create("foo.txt").expect("file not created");
    file.write_all(b"lorem ipsum\ndolor sit amet").expect("Write not performed properly");
    let lines = read_lines(std::path::PathBuf::from("foo.txt")).expect("Lines had error");
    find_matches(lines, "lorem".to_string(), &mut result);
    std::fs::remove_file("foo.txt").expect("File deletion failed");
    assert_eq!(result, b"lorem ipsum\n");
}