// Implement a program that finds the largest number in a vector.

fn get_largest(list: &Vec<u8>) -> u8 {
    let mut largest: u8 = list.first().unwrap().to_owned();

    for i in list {
        if largest < *i {
            largest = *i;
        }
    }

    largest
}

fn main() {
    let list: Vec<u8> = vec![34, 64, 87, 2, 28];
    let largest: u8 = get_largest(&list);
    println!("The largest element in list: {:?} is {}.", list, largest);
}
