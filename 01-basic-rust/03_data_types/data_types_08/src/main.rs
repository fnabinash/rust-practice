// Write a program that casts an integer to a character.

fn main() {
    let num: u32 = 6;
    println!("Number: {}", num);

    let char_num: char = char::from_digit(num, 10).unwrap();
    println!("Char number: {:?}", char_num);
}
