// Write a program that sorts a vector of integers in ascending and descending order.

fn sort_ascending(list: &mut Vec<u8>) -> Vec<u8> {
    list.sort();
    list.to_owned()
}

fn sort_descending(list: &mut Vec<u8>) -> Vec<u8> {
    list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    list.to_owned()
}

fn main() {
    let mut list: Vec<u8> = vec![3, 8, 5, 1, 0];
    println!("List: {:?}", list);
    
    let ascending_list: Vec<u8> = sort_ascending(&mut list);
    let descending_list: Vec<u8> = sort_descending(&mut list);
    
    println!("Ascending list: {:?}", ascending_list);
    println!("Descending list: {:?}", descending_list);
}
