// Create a simple menu-driven program using a loop and match statement.

use std::io;

fn main() {
    let first: i32 = 42;
    let second: i32 = 98;

    println!("\nFirst number: {}, Second number: {} \n", first, second);
    println!("Choose a operation to perform.");
    println!("1. Addition\n2. Substraction\n3. Multiplication\n4. Division");

    let mut operation: String = String::new();
    println!("Enter the number that represnt the operation of your choice: ");

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read the user input!");

    let operation: u8 = operation.trim().parse().expect("Enter a valid operation!");

    match operation {
        1 => println!("Addition of {} and {} is {}", first, second, first + second),
        2 => println!("Substraction of {} from {} is {}", second, first, first - second),
        3 => println!("Multiplication of {} wiht {} is {}", first, second, first * second),
        4 => println!("Division of {} by {} is {}", first, second, first as f32 / second as f32),
        _ => println!("Enter a valid operation!"),
    }
}
