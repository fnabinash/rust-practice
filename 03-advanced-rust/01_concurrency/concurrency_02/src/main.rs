// Implement a program that spawns multiple threads to process a large array and combines the results.

use std::thread;

fn main() {
    static ARR: [u16; 1000] = [8; 1000];

    let arr_chunks = ARR.chunks(10);
    let mut handles = Vec::new();
    let mut total: u16 = 0;

    for chunk in arr_chunks {
        let handle = thread::spawn(|| {
            chunk.iter().sum::<u16>()
        });

        handles.push(handle);
    }

    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("Total: {}", total);
}
