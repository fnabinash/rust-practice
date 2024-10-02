// Create a library crate with a module utils containing utility functions.

mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(utils::is_palindrome("A man a plan a canal Panama"));
    }
    
    #[test]
    fn test_count_vowels() {
        assert_eq!(utils::count_vowels("hello"), 2);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(utils::factorial(5), 120);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(utils::reverse_string("hello"), "olleh");
    }
}


