// Implement a program that calculates the square root of a number.

fn main() {
    let num: u32 = 253;
    let sqrt: f32 = f32::powf(num as f32, 0.5);
    println!("Square root of {} is {}", num, sqrt);
}
