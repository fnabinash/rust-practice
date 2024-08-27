// Implement a program that changes a variable's value based on user input.

use std::io;

fn main() {
    let mut value: u32 = 23;
    println!("Value: {}", value);

    println!("Enter a number: ");

    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the user input!");

    let user_input: u32 = user_input.trim().parse().expect("Failed to parse the string to number!");

    value += user_input;

    println!("User input: {}", user_input);
    println!("New value: {}", value);
}
