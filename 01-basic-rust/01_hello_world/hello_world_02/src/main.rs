// Modify the program to take user input and print "Hello, [name]!".

use std::io;

fn main() {
    let mut name: String = String::new();

    println!("Enter your name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read!");

    println!("Hello, {}!", name.trim());
}
