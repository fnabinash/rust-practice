// Implement a program that filters out even numbers from a vector using an iterator.

fn main() {
    let num_list: Vec<u32> = vec![4, 25, 78, 63, 21, 44, 9];
    println!("The num list: {:?}", num_list);

    let only_evens: Vec<&u32> = num_list.iter()
        .filter(|num| {**num % 2 == 0})
        .collect();

    println!("Only even numbers: {:?}", only_evens);
}
