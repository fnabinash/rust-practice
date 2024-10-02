
use std::{fs, io::{self, Write}};

#[allow(dead_code)]
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

#[allow(dead_code)]
pub fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}

#[allow(dead_code)]
pub fn is_palindrome(input: &str) -> bool {
    let cleaned: String = input.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned.eq_ignore_ascii_case(&reversed)
}

#[allow(dead_code)]
pub fn count_vowels(input: &str) -> usize {
    input.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

#[allow(dead_code)]
pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
