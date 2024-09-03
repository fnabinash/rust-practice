// Implement a program that reads user input and handles errors if the input is not a valid number.

use std::{io, process};

fn main() {
    let mut num: String = String::new();

    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the user input!");

    let num: u32 = num.trim().parse().unwrap_or_else(|e| {
        println!("Enter a vlaid number. {:?} is not a number.\nError: {:?}", num.trim(), e);
        process::exit(1);
    });

    println!("Number: {}", num);
}
