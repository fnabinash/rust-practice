// Convert a number from binary to decimal.

fn main() {
    let binary_num: u32 = 110011;
    println!("Binary number: {}", binary_num);

    let decimal_num: u32 = u32::from_str_radix(binary_num.to_string().as_str(), 2).unwrap();
    println!("Decimal number: {}", decimal_num);
}
