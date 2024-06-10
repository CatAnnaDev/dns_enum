use crossbeam::queue::ArrayQueue;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: Arc<ArrayQueue<Job>>,
    pub(crate) active_jobs: Arc<Mutex<usize>>,
}

enum Job {
    Task(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

impl ThreadPool {
    pub fn new(mut size: usize, capa: usize) -> ThreadPool {
        if size <= 0 {
            size = 1
        }
        let queue = Arc::new(ArrayQueue::<Job>::new(capa));
        let active_jobs = Arc::new(Mutex::new(0));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&queue), Arc::clone(&active_jobs)));
        }
        ThreadPool {
            workers,
            queue,
            active_jobs,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job::Task(Box::new(f));
        {
            let mut active_jobs = self.active_jobs.lock().unwrap();
            *active_jobs += 1;
        }

        if self.queue.push(job).is_err() {
            eprintln!("Queue is full, could not add the job.");
        }

        for worker in &self.workers {
            if let Some(thread) = &worker.thread {
                thread.thread().unpark();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            let _ = self.queue.push(Job::Terminate);
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.thread().unpark();
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(queue: Arc<ArrayQueue<Job>>, active_jobs: Arc<Mutex<usize>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match queue.pop() {
                Some(Job::Task(job)) => {
                    job();
                    let mut active_jobs = active_jobs.lock().unwrap();
                    *active_jobs -= 1;
                }
                Some(Job::Terminate) => break,
                None => thread::park(),
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}
