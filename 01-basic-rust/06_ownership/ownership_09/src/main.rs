// Write a function that takes ownership of a vector and returns its length.

fn main() {
    let num_vec: Vec<u32> = vec![34, 23, 765];
    println!("Vector: {:?}", num_vec);
    
    let length: usize = get_length(num_vec);
    println!("Length of the vector is {}.", length);
}

fn get_length(num_vec: Vec<u32>) -> usize {
    num_vec.len()
}
