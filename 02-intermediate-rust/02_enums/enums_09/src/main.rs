// Write a program that uses an enum to represent different error types (NotFound, PermissionDenied, Unknown) and use pattern matching to handle errors.

use std::fs::File;
use std::io::ErrorKind;

enum ErrorType {
    NotFound(String),
    PermissionDenied(String),
    Unknown(String),
}

fn create_file(file_path: &str) -> Result<File, ErrorType> {
    match File::create(file_path) {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(ErrorType::NotFound(String::from("File path not found."))),
            ErrorKind::PermissionDenied => Err(ErrorType::PermissionDenied(String::from("Permission denied to create the file."))),
            _ => Err(ErrorType::Unknown(format!("An unknown error occurred: {}", error))),
        },
    }
}

fn handle_error(error: ErrorType) {
    match error {
        ErrorType::NotFound(msg) => {
            println!("Error: Not Found - {}", msg);
        }

        ErrorType::PermissionDenied(msg) => {
            println!("Error: Permission Denied - {}", msg);
        }

        ErrorType::Unknown(msg) => {
            println!("Error: Unknown - {}", msg);
        }
    }
}

fn main() {
    let file_path = "file.txt";

    match create_file(file_path) {
        Ok(_) => println!("File created successfully."),
        Err(e) => handle_error(e),
    }
}
