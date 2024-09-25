// Implement a program that uses a closure to filter a vector of strings based on their length.

fn main() {
    let string_list: Vec<&str> = vec!["trait", "impl", "fn", "hello world", "rust", "git", "unsafe"];
    println!("Unsorted strings: {:?}", string_list);

    let mut string_less_than_5: Vec<&str> = Vec::new();
    let mut string_greater_than_5: Vec<&str> = Vec::new();

    string_list.iter()
        .for_each(|str| {
            if str.len() < 5 {
                string_less_than_5.push(str);
            } else {
                string_greater_than_5.push(str);
            }
        });

    println!("String that length is less than 5 are {:?}.", string_less_than_5);
    println!("String that lenght is greather than 5 are {:?}", string_greater_than_5);
}
