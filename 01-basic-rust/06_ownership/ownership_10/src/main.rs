// Create a program that demonstrates the use of references in function parameters and return types.

fn main() {
    let mut text: String = "Hello".to_string();
    let wish_text: &str = wish(&mut text);
    println!("Text: {:?}", wish_text);
}

fn wish(text: &mut String) -> &str {
    text.push_str(" world!");
    text
}
