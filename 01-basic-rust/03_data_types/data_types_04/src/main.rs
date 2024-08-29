// Create a program that determines the type of a variable.

use std::any::type_name_of_val;

fn main() {
    let val: i32 = -6;
    println!("{} has type {}", val, type_name_of_val(&val));
    
    let val: u32 = 5;
    println!("{} has type {}", val, type_name_of_val(&val));

    let val: f32 = 2.5;
    println!("{} has type {}", val, type_name_of_val(&val));

    let val: bool = false;
    println!("{} has type {}", val, type_name_of_val(&val));

    let val: isize = -9;
    println!("{} has type {}", val, type_name_of_val(&val));
}
