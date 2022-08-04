use std::{thread, sync::mpsc};
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        

        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}


enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize ,
    handle: Option<thread::JoinHandle<()>> ,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            handle: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
                
            })),
        }
    }
}

