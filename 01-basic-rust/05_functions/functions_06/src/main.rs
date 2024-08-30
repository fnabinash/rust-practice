// Create a function that checks if a string is a palindrome.

fn main() {
    is_palindrome("Hello, world!");
}

fn is_palindrome(text: &str) {
    let rev_text: String = text.chars().rev().collect::<String>();

    if text == rev_text {
        println!("{:?} is a palindrom.", text);
    } else {
        println!("{:?} is not palindrom.", text);
    }
}
