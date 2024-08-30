// Create a function that returns the square of a number.

fn main() {
    let num: u32 = 8;
    let square: u32 = square(num);
    println!("Square of {} is {}.", num, square);
}

fn square(num: u32) -> u32 {
    num * num
}
