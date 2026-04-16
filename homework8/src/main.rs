use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self};
use rand::{RngExt};

fn assignment3() {
    enum Message {
        NewJob(Job),
        Terminate,
    }
    // Job type is a boxed closure that can be sent across threads
    type Job = Box<dyn FnOnce() + Send + 'static>;

    // ThreadPool struct
    struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Message>,
    }
    impl ThreadPool {
        // Create a new ThreadPool with the specified size
        fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            
            // TODO: Create a channel for sending jobs
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            // TODO: Create and store workers
            let mut workers = vec![];
            for i in 0..size {
                workers.push(Worker::new(i, Arc::clone(&receiver) ));
            }
            let _ = workers[1].id;
            // TODO: Return the ThreadPool
            ThreadPool { workers, sender }
        }
        
        // Execute a job in the thread pool
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            // TODO: Create a job from the closure and send it to a worker
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap(); 
        }
    }

    // Clean up resources when ThreadPool is dropped
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            // TODO: Send terminate message to all workers
            for _ in &self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }
            
            // TODO: Wait for all workers to finish
            for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    // Worker struct represents a thread that can process jobs
    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        // Create a new worker with the specified ID
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
            // TODO: Create a thread that loops and receives jobs from the channel
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} processing a task", id);
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            });
            
            // TODO: Return the Worker
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}
fn assignment4() {
    const TERMINATION_SIGNAL: i32 = -1;

    // TODO: Implement producer function
    fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
        // TODO: Generate random numbers and send them to the channel
        // When finished, producer should NOT send termination signal
        let mut rng = rand::rng();

        for _ in 0..item_count {
            let value = rng.random_range(1..100);

            println!("Producer {} produced the number {}", id, value);
            tx.send(value).unwrap();
        }
        println!("Producer {} done", id)
    }

    // TODO: Implement consumer function
    fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
        // TODO: Receive numbers from the channel and process them
        // Break the loop when receiving the termination signal
        loop {
            let receiver = rx.lock().unwrap();
            let value = receiver.recv();
            match value {
                Ok(num) => {
                    if num == TERMINATION_SIGNAL {
                        println!("Consumer {} terminated", id);
                        break;
                    } else {
                        println!("Consumer {} processed the number {}", id, num);
                    }
                }
                Err(_) => {
                    println!("Consumer {}: channel closed", id);
                    break;
                }
            }
        }
        println!("Consumer {} done", id)
    }
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut producers = vec![];
    for i in 0..2 {
        let tx_clone = tx.clone();
        let items_per_producer = ITEM_COUNT / 2;

        let handle = thread::spawn(move || {
            producer(i, tx_clone, items_per_producer);
        });

        producers.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let mut consumers = vec![];
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);

        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });

        consumers.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in producers {
        handle.join().unwrap();
    }
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    for handle in consumers {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

fn main() {
    assignment3();
    assignment4();
}
