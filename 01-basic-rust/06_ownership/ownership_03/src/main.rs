// Write a function that takes two borrowed strings and concatenates them.

fn main() {
    let first: &str = "Hello";
    let second: &str = " world!";
    let concat_str: String = concat(first, second);

    println!("{:?} + {:?} = {:?}", first, second, concat_str);
}

fn concat(first: &str, second: &str) -> String {
    first.to_string() + second
}
