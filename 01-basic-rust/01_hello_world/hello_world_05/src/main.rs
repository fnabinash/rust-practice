// Format a string with multiple variables and print it.

fn main() {
    let name: &str = "Rama";
    let age: u8 = 34;
    let address: String = String::from("Rust Programming");
    let is_right: bool = false;

    println!("My name is {}, of age {} and learing {}. This statement is {}.", name, age, address, is_right);
}
