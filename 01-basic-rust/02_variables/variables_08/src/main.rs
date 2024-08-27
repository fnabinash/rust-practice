// Create a program that doubles the value of a number using a mutable variable.

fn main() {
    let mut value: u8 = 78;
    println!("Value: {}", value);

    value *= 2;
    println!("Doubled Value: {}", value);
}
