// Write a program that handles file reading errors and displays an appropriate message if the file is not found.

use std::{fs, io::Error};

fn read_file(path: &str) -> Result<String, Error>{
    fs::read_to_string(path)
}

fn main() {
    let path: &str = "text.txt";
    match read_file(path) {
        Ok(content) => println!("File contents: {:?}", content),
        Err(_) => println!("File {:?} your are searching for is not found.", path),
    }
}
