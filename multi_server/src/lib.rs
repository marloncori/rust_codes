use std::error::Error;
use std::{fmt, thread}; 
use std::{time::Duration, sync::mpsc};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct EmptyPoolError;
impl Error for EmptyPoolError {}
impl fmt::Display for EmptyPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\t Thread pool size cannot be equal to zero!")
    }
}

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, EmptyPoolError> {
        match size {
           0  => Err(EmptyPoolError),
           _  => {
                let (tx, rx) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(rx)); 
                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, 
                        Arc::clone(&receiver)));
                }
                Ok(ThreadPool {
                    workers: workers,
                    sender: tx,
                })
             },
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        // an execute is called, a channel will send the load
        // to my worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// cleanup code
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("\n\t Attention: Server has got an order to clean up!");
        
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        let line = String::from("\n\t*********************************\n");

        println!("{}\t   Halting down all thread workers!{}", &line, line);
        for worker in &mut self.workers {
            println!("\n\t[WEB INFO] ===> Disconnecting worker n. {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); //the None variant stays stored
            }
        }
    }
}
// end of cleanup code

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let line = String::from("\n\t===============================================\n");
        let thread = thread::spawn(move || loop { 
            let msg = rx.lock().unwrap().recv().unwrap();
            match msg {
               Message::NewJob(job) => {
                        println!("{}\t\tThread worker #{} has a job!\n\t\tHe is busy executing it...{}", &line, id, line);
                        thread::sleep(Duration::from_millis(2500));
                        job();
                      },
               Message::Terminate => {
                        let line2 = String::from("\n\t:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::\n");
                        println!("{}\t   Oops... worker #{} has been told to terminate.\n\t\t Shutting down...{}", &line2, id, line2);
                        thread::sleep(Duration::from_millis(500));
                        break;
                     },
            }
        });
        Worker {
            id, thread: Some(thread),
        }
    }
}