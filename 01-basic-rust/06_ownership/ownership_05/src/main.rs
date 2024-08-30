// Implement a function that modifies a borrowed string using mutable references.

fn main() {
    let origina_text: &str = "Original Text";
    modify_str(origina_text);
}

fn modify_str(mut text: &str) {
    println!("Original text: {:?}", text);
    text = "Changed Text";
    println!("Changed text: {:?}", text);
}
