// Write a program that uses threads to perform multiple calculations in parallel.

use std::{thread, time::Duration};

fn main() {
    println!("Main thread start.");

    let thread1 = thread::spawn(||{
        println!("Thread1 start.");
        let mut sum: u32 = 0;
        
        for i in 1..1001 {
            sum += i;
            thread::sleep(Duration::from_millis(1));
        }
        
        println!("Sum of 1 to 1000 is {}.", sum);
        println!("Thread1 end.");
    });

    let thread2 = thread::spawn(|| {
        println!("Thread2 start.");

        println!("Doing somthing big calculation.");
        thread::sleep(Duration::from_micros(500));

        println!("Thread2 end.");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Main thread end.");
}
