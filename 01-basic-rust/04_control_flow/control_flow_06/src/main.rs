// Implement a program that finds the largest of three numbers.

fn main() {
    let first: u32 = 35;
    let second: u32 = 28;
    let three: u32 = 92;
    let mut biggest: u32 = first;

    if second > first {
        if second > three {
            biggest = second;
        } else {
            biggest = three;
        }
    } else if three > first {
        biggest = three;
    }

    println!("Biggest number: {}", biggest);
}
