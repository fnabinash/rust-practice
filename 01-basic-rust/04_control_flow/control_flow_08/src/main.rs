// Write a program that checks if a year is a leap year.

use std::io;

fn main() {
    let mut year: String = String::new();

    println!("Enter a year: ");

    io::stdin()
        .read_line(&mut year)
        .expect("Failed to read the user input!");

    let year: u32 = year.trim().parse().expect("Enter a valid year!");

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("{} is a leap year.", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}
