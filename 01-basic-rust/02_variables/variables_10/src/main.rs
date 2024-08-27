// Create a program that increments a counter variable each time a button is pressed.

use std::io;

fn main() {
    let mut value: u8 = 0;
    println!("Initial value: {}\n", value);

    println!("Press \"Enter\" to increment or type \"exit\" and press \"Enter\" to exit the program.");

    loop {
        let mut exit: String = String::new();

        io::stdin()
            .read_line(&mut exit)
            .expect("Failed to read the user input");

        if exit.trim() == "exit" {
            break;
        }

        value += 1;

        println!("Updated value: {}", value);
    }

    println!("Final value is: {}", value);
}
