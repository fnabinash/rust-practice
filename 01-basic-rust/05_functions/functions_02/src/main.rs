// Implement a function that takes two numbers and returns their sum.

fn main() {
    let first: u32 = 432;
    let second: u32 = 243;

    let sum: u32 = sum(first, second);
    println!("Sum of {} and {} is {}.", first, second, sum);
}

fn sum(first: u32, second: u32) -> u32 {
    first + second
}
