// Create a program that uses channels to send messages between threads.

use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, reciever) = mpsc::channel();

    let consumer_thread: thread::JoinHandle<()> = thread::spawn(move || {
        for msg in reciever.iter() {
            println!("Recieved {}", msg);
        }
    });

    for i in 0..10 {
        sender.send(i).expect("Failed to send!");
        thread::sleep(Duration::from_millis(500));
    }

    drop(sender);

    consumer_thread.join().expect("Panic occured in the consumer thread!");
}
