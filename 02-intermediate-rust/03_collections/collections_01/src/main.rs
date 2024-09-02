// Create a program that adds and removes elements from a vector.

fn main() {
    let mut list: Vec<u32> = vec![1, 2, 4, 5];
    println!("List: {:?}", list);

    list.push(6);
    println!("After adding, List: {:?}", list);

    list.pop();
    list.pop();
    println!("Removing last 2 elements, List: {:?}", list);

    list.remove(1);
    println!("Removing middle element, List: {:?}", list);
}
