// Create a module to organize functions for mathematical operations and use it in a main program.

mod operations;

fn main() {
    let first: i32 = 23;
    let second: i32 = -14;

    println!("Addition of {} and {} is {}.", first, second, operations::add(first, second));
    println!("Subtraction of {} from {} is {}.", second, first, operations::subtract(first, second));
    println!("Multiplication of {} and {} is {}.", first, second, operations::multiply(first, second));
    println!("Division of {} by {} is {}", first, second, operations::divide(first, second));
}
