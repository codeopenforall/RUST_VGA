use std::sync::{Arc, Mutex};

struct Conn {
    id: u64,
}

struct Server {
    conns: Arc<Mutex<Vec<Conn>>>,
}

impl ResourceManager for Server {
    fn new(_limit: usize) -> Self {
        Server {
            conns: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn run(&self) {
        let conns = self.conns.clone();
        let mut id_counter = 0u64;
        loop {
            unsafe {
                let connection = Conn { id: id_counter };
                let ptr: *mut Conn = Box::into_raw(Box::new(connection));
                (*conns.lock().unwrap()).push(*Box::from_raw(ptr));
                id_counter = id_counter.wrapping_add(1);
            }
            // Simulate some work
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn get_count(&self) -> usize {
        self.conns.lock().unwrap().len()
    }
}

pub type ResourceImpl = Server;

trait ResourceManager {
    fn new(limit: usize) -> Self;
    fn run(&self);
    fn get_count(&self) -> usize;
}