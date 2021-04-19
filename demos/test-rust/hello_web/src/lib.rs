use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    // 使用 Worker 来存储 JoinHandle
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);
        // 在不同的线程，如何分配 Job 并且使用一个 Receiver?
        // 线程安全的处理 Receiver
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // move 报错
            //workers.push(Worker::new(id, receiver));
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }
        ThreadPool { workers, sender }
    }
    // 参考 spawn 的定义
    pub fn execute<F>(&self ,f: F)
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
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // execute 调用该怎么做?
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}
