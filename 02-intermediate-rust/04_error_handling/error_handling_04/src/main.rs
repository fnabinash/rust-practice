// Write a function that attempts to open a file and returns a custom error type if the file cannot be opened.

use std::{fs::{self, File}, io::Error};

fn read_file(path: &str) -> Result<File, Error> {
    fs::File::open(path)
}

fn main() {
    let path: &str = "text.txt";
    match read_file(path) {
        Ok(content) => println!("File contents: {:?}", content),
        Err(err) => println!("Can not open the file {:?}. Reason: {}", path, err),
    }
}
