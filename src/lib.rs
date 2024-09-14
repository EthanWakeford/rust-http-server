use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<JobReceiver>,
}

impl Worker {
    fn new(id: usize, receiver: JobReceiver) -> Worker {
        // let thread = thread::spawn(|| {receiver});

        let thread = thread::spawn(move || loop {
            // Call to recv blocks until job is passed
            // lock ensures only one worker is waiting for a job at a given moment
            // mutex guard is implicitly dropped after at the end of this statement as it is not the final value
            let job = receiver.lock().expect("Mutex Poisoned").recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a ThreadPool
    ///
    /// The size is the number of threads
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
