use std::{f64::consts::PI, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

use rand::random;

use crate::vec3::Vec3;

pub fn _random_range(min: f64, max: f64) -> f64 {
    (max-min)*random::<f64>() + min
}

pub fn sample_square() -> Vec3 {
    Vec3::new(random::<f64>()-0.5, random::<f64>()-0.5, 0.0)
}

pub fn degrees_to_radians(x: f64) -> f64 {
    x * PI / 180.0
}

type Job = Vec<Box<dyn FnOnce() + Send + 'static>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self { workers, sender: Some(sender)}
    }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: Vec<F>) {
        let mut job: Job = Vec::with_capacity(f.len()+1);

        for j in f {
            job.push(Box::new(j));
        }

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    _id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let recv = receiver.lock().unwrap().recv();
            match recv {
                Ok(list) => {
                    for job in list {
                        job();
                    }
                },
                Err(_) => {
                    break;
                },
            }
        });

        Self { _id, thread: Some(thread) }
    }
}