// Implement a program that uses a module to organize functions for file handling and use it in a main program.

use std::process;

mod file_ops;

fn main() {
    let filename = "example.txt";

    if let Err(err) = file_ops::create_file(filename) {
        eprintln!("Error creating file: {}", err);
        process::exit(1);
    }

    match file_ops::read_file(filename) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        },
    }

    if let Err(err) = file_ops::append_file(filename, " This is some additional text.\n") {
        eprintln!("Error appending to file: {}", err);
        process::exit(1);
    }

    match file_ops::read_file(filename) {
        Ok(contents) => println!("Updated file contents: {}", contents),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        },
    }

    if let Err(err) = file_ops::delete_file(filename) {
        eprintln!("Error deleting file: {}", err);
        process::exit(1);
    }
}
