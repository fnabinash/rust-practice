// Parse a string as a number and perform arithmetic on it.

fn main() {
    let num: &str = "12";
    println!("String: {}", num);

    let num: u32 = num.parse().unwrap();
    println!("Number: {}", num);
    println!("Square of the number: {}", num * num);
}
