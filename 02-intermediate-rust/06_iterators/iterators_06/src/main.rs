// Write a program that chains multiple iterators to process a vector of numbers.

fn main() {
    let mut num_list: Vec<u8> = vec![5, 6, 1, 9, 87, 21, 3, 112, 5];
    println!("Numbers: {:?}", num_list);

    num_list.iter_mut()
        .for_each(|num| {
            if *num % 2 != 0 {
                *num *= 2;
            }
        });

    println!("All numbers are even now. Numers: {:?}", num_list);
}
