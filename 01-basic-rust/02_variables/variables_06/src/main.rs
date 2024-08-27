// Convert a string to an integer and print the result.

fn main() {
    let the_string: &str = "789";
    println!("String: {:?}", the_string);

    let the_number: u32 = the_string.parse().unwrap();
    println!("Number: {:?}", the_number);
}
