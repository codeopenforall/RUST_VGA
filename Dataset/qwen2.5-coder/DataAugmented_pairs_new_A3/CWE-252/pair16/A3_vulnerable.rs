use std::thread;
use std::sync::mpsc;

struct Worker {
    sender: mpsc::Sender<()>,
}

impl Worker {
    fn execute(&self) -> Result<(), String> {
        self.sender.send(()).map_err(|_| "Failed to send message".to_string())
    }
}

fn run() {
    let (sender, receiver) = mpsc::channel();
    let worker = Worker { sender };

    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            let _ = worker.execute(); // Intentionally ignoring the Result
        }
    });

    handle.join().unwrap();
}

fn main() {
    run();
}