// Write a program that creates a module to organize functions for string processing and use it in a main program.

mod str_ops;

fn main() {
    let my_string = "Hello, Rust!";
    println!("Original: {}", my_string);
    
    let reversed = str_ops::reverse_string(my_string);
    println!("Reversed: {}", reversed);

    let uppercased = str_ops::to_uppercase(my_string);
    println!("Uppercased: {}", uppercased);

    let vowel_count = str_ops::count_vowels(my_string);
    println!("Vowel count: {}", vowel_count);
}
