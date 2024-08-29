// Write a program that checks if a number is positive, negative, or zero.

fn main() {
    let num: i32 = -57;

    if num.is_negative() {
        println!("{} is negative.", num);
    } else if num.is_positive() {
        println!("{} is positive.", num);
    } else {
        println!("{} is zero.", num);
    }
}
