// Write a program that uses Arc (atomic reference counting) to share data between threads safely.

use std::{sync::Arc, thread};

fn main() {
    let nums: Arc<Vec<u8>> = Arc::new(vec![4, 2, 8, 32]);

    thread::scope(|s| {
        s.spawn(|| {
            for i in Arc::clone(&nums).iter() {
                print!("{} ", i);
            }
        });

        s.spawn(|| {
            println!("\nLength: {}", Arc::clone(&nums).len());
        });
    });
}
