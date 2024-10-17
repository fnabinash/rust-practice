// Create a program that uses Mutex to protect shared data from race conditions.

use std::{sync::{Mutex, Arc}, thread};

fn main() {
    let nums: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![1, 2, 3, 4]));

    let mut handles = Vec::new();

    for i in 0..5 {
        let thread = thread::spawn({
            let nums = Arc::clone(&nums);
            
            move || {
                println!("Thread {} got the lock.", i);
                let mut lock = nums.lock().expect("failed to get the lock.");
                lock.push(5 + i);
            }
        });

        handles.push(thread);
    }

    for handle in handles {
        handle.join().expect("Failed to join the handle.");
    }

    println!("{:?}", nums.lock().expect("Failed to get the lock"));
}
