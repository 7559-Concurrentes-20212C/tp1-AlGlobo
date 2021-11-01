use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::RecvError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    pub sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(reciver));

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

        self.sender.send(Message::NewJob(job)).expect("Error in sending Job to Worker"); //TODO chequear si no habria que manejar el error o burbujearlo
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).expect("Error in sending Job to Worker"); //TODO chequear si no habria que manejar el error o burbujearlo
        }

        //println!("Shutting down all workers.");

        for worker in &mut self.workers {
            //println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() { // TODO take seria como un unwrap chequear
                thread.join().expect("there was an error joining the workings");
            }
        }
    }
}

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            
            let message = match receiver.lock().expect("poisoned lock").recv() {
                Ok(msg) => {msg}
                Err(_) => {return}
            };

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread), }
    }
}