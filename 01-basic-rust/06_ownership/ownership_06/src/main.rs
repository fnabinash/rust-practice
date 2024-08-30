// Write a program that demonstrates the use of multiple references.

fn main() {
    let arr_str: [&str; 5] = ["hi", "hello", "bye", "done", "ok"];
    make_one_str(&arr_str);
}

fn make_one_str(arr_str: &[&str; 5]) {
    let mut one_str: String = String::new();

    for text in arr_str {
        one_str.push_str(text);
    }

    println!("Arrays of String: {:?}", arr_str);
    println!("Complete String: {:?}", one_str);
}
