// Write a function that calculates the greatest common divisor (GCD) of two numbers.

fn main() {
    let first: u32 = 330;
    let second: u32 = 750;

    let lcm: u32 = find_lcm(first, second);
    let gcd: u32 = find_gcd(first, second, lcm);
    println!("GCD of {} and {} is {}", first, second, gcd);
}

fn find_lcm(first: u32, second: u32) -> u32 {
    let mut lcm: u32 = first * second;

    for i in 1..first {
        if second * i % first == 0 {
            lcm = second * i;
            break;
        }
    }

    lcm
}

fn find_gcd(first: u32, second: u32, lcm: u32) -> u32 {
    first * second / lcm
}
