use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // This line unlocks immediately after the LockResult's unwrap().
            // The `Mutex` struct has no public `unlock` method because the ownership
            // of the lock is based on the lifetime of the `MutexGuard<T>` within the
            // `LockResult<MutexGuard<T>>` that the lock method returns.  At compile
            // time, the borrow checker can then enforce the rule that a resource guarded
            // by a `Mutex` cannot be accessed unless we hold the lock.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();

            // Bad implementation.
            // This implementation can result in the lock being held longer than intended
            // if we don't think carefully about the lifetime of the `MutexGuard<T>`.
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);

            //     job;
            // }
        });

        Worker {id, thread }
    }
}
