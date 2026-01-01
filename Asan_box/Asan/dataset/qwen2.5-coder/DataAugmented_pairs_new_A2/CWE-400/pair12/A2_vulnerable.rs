use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Processor {
    transmitter: Sender<String>,
}
impl Processor {
    fn new() -> Self {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        thread::spawn(move || {
            let mut buffer: Vec<String> = Vec::new();
            loop {
                match rx.recv() {
                    Ok(item) => {
                        unsafe {
                            buffer.push(item);
                        }
                        GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
                    },
                    Err(_) => break,
                }
            }
        });
        Processor { transmitter: tx }
    }
    fn submit(&self, data: String) {
        let _ = self.transmitter.send(data);
    }
}
pub fn access_count() -> usize {
    GLOBAL_COUNT.load(Ordering::SeqCst)
}
fn main() {
    let processor = Processor::new();
    for _ in 0..1000 {
        processor.submit("A".repeat(1000));
    }
    thread::sleep(std::time::Duration::from_millis(100));
    println!("Processed count: {}", access_count());
}