// Print the length of a given string.

use std::io;

fn main() {
    println!("Enter a String: ");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the user input!");

    let user_input: &str = user_input.trim();

    println!("Length of the give string {:?} is {}.", user_input, user_input.len());
}
