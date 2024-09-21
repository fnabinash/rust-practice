// Create a program that uses an iterator to sum all elements in a vector.

fn main() {
    let num_list: Vec<u32> = vec![5, 78, 6, 8, 42, 36];
    println!("Num list: {:?}", num_list);

    let sum: u32 = num_list.iter().sum();
    println!("Sum of all numbers in {:?} is {}.", num_list, sum);
}
