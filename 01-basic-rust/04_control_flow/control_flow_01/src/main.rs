// Implement a program that checks if a number is even or odd.

fn main() {
    let num: u32 = 34;
    let is_even: bool = num % 2 == 0;

    if is_even {
        println!("{} is even.", num);
    } else {
        println!("{} is odd.", num);
    }
}
