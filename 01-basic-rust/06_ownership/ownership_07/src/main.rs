// Create a function that borrows an array and returns its sum.

fn main() {
    let arr: [u32; 5] = [4, 6, 2, 3, 7];
    let sum: u32 = count_sum(&arr);
    println!("Sum of all elements of {:?} is {}.", arr, sum);
}

fn count_sum(arr: &[u32; 5]) -> u32 {
    let mut sum: u32 = 0;

    for i in arr {
        sum += i;
    }

    sum
}
