// Implement a program that uses a HashMap to count the occurrences of words in a text.

use std::collections::HashMap;

fn main() {
    let text: &str = "Hello, world!";
    let mut char_count: HashMap<char, u8> = HashMap::new();

    text.chars().for_each(|c| {
        let count = char_count.entry(c).or_default();
        *count += 1;
    });

    println!("Count list: {:?}", char_count);
}
