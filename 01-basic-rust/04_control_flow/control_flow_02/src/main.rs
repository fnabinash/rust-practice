// Write a simple program that uses if-else to compare two numbers.

fn main() {
    let first_number: u32 = 34;
    let second_number: u32 = 63;

    if first_number > second_number {
        println!("First number {} is bigger than second number {}.", first_number, second_number);
    } else {
        println!("Second number {} is bigger than first number {}.", second_number, first_number);
    }
}
