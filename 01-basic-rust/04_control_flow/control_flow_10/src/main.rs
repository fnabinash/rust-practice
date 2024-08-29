// Write a program that calculates the factorial of a number using a loop.

use std::io;

fn main() {
    let mut num: String = String::new();

    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the user input!");

    let num: u32 = num.trim().parse().expect("Enter a valid number");

    let mut fact: u32 = 1;

    for i in 1..=num {
        fact *= i;
    }

    println!("Factorial of {} is {}", num, fact);
}
