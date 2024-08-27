// Swap two variables without using a third variable.

use std::mem::swap;

fn main() {
    let mut first_number: u8 = 89;
    let mut second_number: u8 = 34;

    println!("First variable: {}, Second variable: {}", first_number, second_number);
    
    swap(&mut first_number, &mut second_number);
    println!("First variable: {}, Second variable: {}", first_number, second_number);
}
