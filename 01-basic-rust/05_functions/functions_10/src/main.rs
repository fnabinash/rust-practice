// Create a function that generates a random number within a specified range.

use rand::Rng;

fn main() {
    let random_num: u32 = get_random_num();
    println!("The random number is {}", random_num);
}

fn get_random_num() -> u32 {
    rand::thread_rng().gen_range(0..100)
}
