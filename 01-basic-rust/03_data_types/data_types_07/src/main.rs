// Convert a number to a string and concatenate it with another string.

fn main() {
    let num: u32 = 4;
    println!("Number: {}", num);

    let num_str: String = num.to_string();
    println!("String number: {:?}", num_str);

    let new_num_str: String = num_str + " Hello World!";
    println!("Concatenated String number: {:?}", new_num_str);
}
