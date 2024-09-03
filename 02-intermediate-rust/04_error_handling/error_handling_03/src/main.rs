// Create a program that parses a string as an integer and handles invalid input with error messages.

use std::process;

fn main() {
    let num: String = String::from("gf");

    let num: u32 = num.parse().unwrap_or_else(|e| {
        println!("{:?} is not a number. Error: {:?}", num, e);
        process::exit(1);
    });

    println!("Number: {}", num);
}
