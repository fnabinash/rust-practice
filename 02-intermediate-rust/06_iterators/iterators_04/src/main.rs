// Create a program that uses an iterator to find the first element in a vector that satisfies a condition.

use std::process;

fn main() {
    let num_list: Vec<u8> = vec![4, 2, 3, 7, 1, 4, 6];
    println!("The number list: {:?}", num_list);

    let num: &u8 = num_list.iter()
        .find(|num| {
            **num >= 5
        }).unwrap_or_else(|| {
            eprintln!("There is no such number that >= 5.");
            process::exit(1);
        });

    println!("First number that >= 5 is {}", num);
}
