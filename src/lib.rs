use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
  // thread pool to store a vector of workers
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

enum Message {
  NewJob(Job),
  Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    // if size is less than one our func will panic
    assert!(size > 0);

    let (sender, reciever) = mpsc::channel();
    
    let mut workers = Vec::with_capacity(size);

    let receiver = Arc::new(Mutex::new(reciever));
    for id in 0..size {
      // create threads
      workers.push(Worker::new(
        id, 
        Arc::clone(&receiver)
      ));
    }

    ThreadPool {workers, sender}
  }

  // we want execute func to similar to spawn func
  // first arg to execute func is gonna be a ref to self bc it is a method, second arg is gonna be a generic that has that
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
    println!("Sending terminate message to all workers.");

    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Shutting Down Worker {}", worker.id);

      // calling the take bc it returns option and matches with some var
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap()
      }
    } 
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>
}

impl Worker {
  fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = reciever.lock().unwrap().recv().unwrap(); 
      
      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);

          job();
        }

        Message::Terminate => {
          println!("Worker {} was told to terminate T.T", id);

          break;
        }
      }
    });

    Worker { id, thread: Some(thread) }
  }
}