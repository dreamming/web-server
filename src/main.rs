use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    thread::sleep(Duration::from_millis(1500));
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}




use std::{sync::{Arc, Mutex, mpsc}};
use std::time::Duration;

enum Message {
    NewJob(Job),
    Terminate,
}
struct Worker {
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker {
     fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
           let message = receiver.lock().unwrap().recv().unwrap();
           match message {
               Message::NewJob(job) => {
                 println!("Worker {} got a job; executing.",id);
                 job();
               }
               Message::Terminate => {
                   println!("Worker {} was told to terminate.",id);
                   break;
               }
           }
           
        });
        Worker {
            id,
            thread:Some(thread)
        }
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender:mpsc::Sender<Message>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for n in 0..size {
            workers.push(Worker::new(n,Arc::clone(&receiver)));
        }
        ThreadPool { workers,sender }
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
            println!("Shutting down worker {}",worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
