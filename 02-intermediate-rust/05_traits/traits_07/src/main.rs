// Implement a trait called Readable with a method read and implement it for different file types.

use std::{fs, process};

trait Readable {
    fn read(&self);
}

struct File {
    path: String
}

impl Readable for File {
    fn read(&self) {
        let contents: String = fs::read_to_string(self.path.as_str()).unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            process::exit(1);
        });

        println!("File contents: {:?}", contents);
    }
}

fn main() {
    let file: File = File { path: "traits_07/Cargo.toml".to_string() };
    file.read();
}
