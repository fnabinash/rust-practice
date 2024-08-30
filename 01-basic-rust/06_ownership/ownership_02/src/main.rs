// Implement a function that borrows a string and returns its length.

fn main() {
    let text: &str = "Hello World!";
    let length: usize = get_len(text);
    println!("Length of {:?} is {}", text, length);
}

fn get_len(text: &str) -> usize {
    text.len()
}
