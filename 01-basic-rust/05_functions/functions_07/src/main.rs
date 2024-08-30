// Implement a function that returns the maximum of two numbers.

fn main() {
    let first: u32 = 52;
    let second: u32 = 32;
    let biggest: u32 = biggest(first, second);
    println!("The biggest number between {} and {} is {}", first, second, biggest);
}

fn biggest(first: u32, second: u32) -> u32 {
    let mut biggest: u32 = first;

    if second > first {
        biggest = second;
    }

    biggest
}
