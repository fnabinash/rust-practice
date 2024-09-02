// Create a program that removes duplicate elements from a vector.

fn remove_duplicates(list: &mut Vec<u8>) -> Vec<u8> {
    list.sort();
    list.dedup();
    list.to_owned()
}

fn main() {
    let mut list: Vec<u8> = vec![1, 2, 5, 3, 2, 2, 3, 1, 1];
    println!("List: {:?}", list);

    let removed_list: Vec<u8> = remove_duplicates(&mut list);
    println!("After removing duplicate items, List: {:?}", removed_list);
}
