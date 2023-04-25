use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{thread_pool::Message};

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} has taken a job! executing..", id);
                    job();
                },
                Message::Terminate => {
                  break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
