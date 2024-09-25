// Write a program that uses a closure to find the maximum value in a vector of integers.

use std::process;

fn main() {
    let numbers: Vec<u32> = vec![4, 5, 45, 13, 78, 12, 6, 3];
    println!("Number list: {:?}", numbers);

    let max_num: u32 = numbers.into_iter().max().unwrap_or_else(|| {
        eprintln!("The number list is empty.");
        process::exit(1);
    });

    println!("The maximum number in the list is {}", max_num);
}
