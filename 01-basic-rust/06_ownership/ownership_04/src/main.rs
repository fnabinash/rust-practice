// Create a function that returns ownership of a string to the caller.

fn main() {
    let text: String = get_ownership();
    println!("Text: {:?}", text);
}

fn get_ownership() -> String {
    let text: &str = "Hello world!";
    text.to_string()
}
