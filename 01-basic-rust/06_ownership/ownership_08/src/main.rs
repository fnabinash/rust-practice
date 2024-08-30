// Implement a program that shows how ownership changes when a variable is reassigned.

fn main() {
    let text1: String = "Hello, world!".to_string();
    println!("Text1: {:?}", text1);

    let text2: String = text1;
    println!("Text1 is no longer valid. Text2: {:?}", text2);
}
