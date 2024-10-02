// Create a binary crate that uses the library crate you made.

use modules_06::utils;

fn main() {
    match utils::write_to_file("./modules_07/example.txt", "Hello, Rust!") {
        Ok(_) => println!("File written successfully."),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    let reversed = utils::reverse_string("world");
    println!("Reversed: {}", reversed);

    let palindrome = utils::is_palindrome("A man a plan a canal Panama");
    println!("Is palindrome: {}", palindrome);

    let vowel_count = utils::count_vowels("Hello, world!");
    println!("Vowel count: {}", vowel_count);

    let factorial_of_5 = utils::factorial(5);
    println!("Factorial of 5: {}", factorial_of_5);
}
