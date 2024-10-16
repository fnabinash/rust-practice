// Implement a program that uses a thread pool to manage and execute multiple tasks concurrently.

use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread, time::Duration};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "Need at least 1 worker!");

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&reciever)));
        }

        Self {
            workers,
            sender: Some(sender)
        }
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(job);
        self.sender.as_ref()
            .expect("Can not find the sender!")
            .send(job)
            .expect("Failed to send the job to workers!");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Thread {} is shutting down.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect(format!("Failed to join the thread {}", worker.id).as_str());
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let receiver = reciever.lock()
                    .expect("Failed to grab the lock!")
                    .recv();

                match receiver {
                    Ok(job) => {
                        println!("Thread {} got the job & executing.", id);
                        job();
                        thread::sleep(Duration::from_millis(10));
                    },

                    Err(_) => {
                        println!("No got the job");
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread)
        }
    }
}

fn main() {
    let pool = ThreadPool::new(5);

    for _ in 0..10 {
        pool.execute(|| {
            println!("Doing something")
        });
    }
}