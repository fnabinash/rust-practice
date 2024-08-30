// Create a function that takes ownership of a string and prints it.

fn main() {
    take_ownership("Hello, world!".to_string());
}

fn take_ownership(text: String) {
    println!("{}", text);
}
