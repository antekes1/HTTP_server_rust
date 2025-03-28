use std::{sync::{mpsc::{self}, Arc, Mutex}, thread};
pub mod handlers;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    // Create a new Theread Pool.
    // The size is the number of threads in pool.
    // Panics when size is 0 and less
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, recever) = 
            mpsc::channel();

        let receiver = 
            Arc::new(Mutex::new(recever));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sendling message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);
                    job(); 
                },
                Message::Terminate => {
                    println!("Worker {} was told to shut down; tarminating", id);
                    break;
                }
            }
        });

        Worker{ id, thread: Some(thread) }
    }
}