// Write a program that maps a vector of integers to their squares using an iterator.

fn main() {
    let mut num_list: Vec<u32> = vec![45, 8, 12, 74, 36, 64, 3];
    println!("The vector of integers is: {:?}", num_list);

    num_list.iter_mut()
        .for_each(|num| {
            *num = u32::pow(*num, 2);
        });

    println!("The squares of numbers are: {:?}", num_list);
}
