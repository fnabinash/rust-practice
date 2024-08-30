// Write a function that reverses a string.

fn main() {
    let text: &str = "Hello";
    let rev_text: String = revers(text);
    println!("Reverse of {:?} is {:?}", text, rev_text);
}

fn revers(text: &str) -> String {
    text.chars().rev().collect::<String>()
}
