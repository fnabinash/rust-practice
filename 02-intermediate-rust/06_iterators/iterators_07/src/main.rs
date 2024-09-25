// Create a program that uses a closure to sort a vector of integers in descending order.

fn main() {
    let mut numbers: Vec<u8> = vec![4, 2, 8, 6, 3, 9, 10, 7, 1];
    println!("Unsorted numbers: {:?}", numbers);

    numbers.sort_by(|a, b| {
        b.cmp(a)
    });

    println!("Sorted numbers: {:?}", numbers);
}
