pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}
