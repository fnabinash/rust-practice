// Implement a function that calculates the sum of an array of numbers.

fn main() {
    let arr: [u32; 5] = [1, 4, 2, 53, 23];
    let sum: u32 = sum(arr);
    println!("The sum of array: {:?} is {}.", arr, sum);
}

fn sum(arr: [u32; 5]) -> u32 {
    let mut sum: u32 = 0;

    for i in arr {
        sum += i;
    }

    sum
}
