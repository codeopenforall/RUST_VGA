use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender};
const SIZE: usize = 10 * 1024 * 1024; 

struct Worker {
    transmitter: Sender<Vec<u8>>,
}

impl Worker {
    fn initialize(transmitter: Sender<Vec<u8>>) -> Self {
        Worker { transmitter }
    }

    fn operate(&self) {
        let transmitter_clone = self.transmitter.clone();
        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(SIZE);
            unsafe {
                buffer.set_len(SIZE);
            }
            transmitter_clone.send(buffer).unwrap();
        });
    }
}

fn dispatch_tasks(task_count: usize) {
    let (tx, rx) = channel();
    let worker = Worker::initialize(tx);
    for _ in 0..task_count {
        worker.operate();
    }
    for _ in 0..task_count {
        let _ = rx.recv().unwrap();
    }
}

#[no_mangle]
pub fn run_app(tasks: usize) -> Result<(), &'static str> {
    dispatch_tasks(tasks);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let job_count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_app(job_count).unwrap();
    println!("Processing finished");
}