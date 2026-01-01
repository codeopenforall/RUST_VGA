use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};

struct Handler {
    sender: Sender<String>,
}

impl Handler {
    pub fn new() -> Self {
        let (tx, _rx): (Sender<String>, Receiver<String>) = channel();
        Handler { sender: tx }
    }

    pub fn submit(&self, data: String) {
        self.sender.send(data).unwrap();
    }
}

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn access_count() -> usize {
    GLOBAL_COUNT.load(Ordering::SeqCst)
}