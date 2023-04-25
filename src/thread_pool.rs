use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex, RwLock};


use crate::job::Job;
use crate::worker::{Worker};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

pub enum Message{
    Terminate,
    NewJob(Job)
  }

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = channel();


        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 1..size + 1 {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers{
            println!("Shutting down worker.. {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
