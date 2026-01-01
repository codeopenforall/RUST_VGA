use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
const MAX_STORAGE: usize = 100;
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Handler {
    sender: SyncSender<String>,
}
impl Handler {
    fn new() -> Self {
        let (tx, rx): (SyncSender<String>, Receiver<String>) = sync_channel(1000);
        thread::spawn(move || {
            let mut storage: Vec<String> = Vec::new();
            loop {
                match rx.recv() {
                    Ok(data) => {
                        if storage.len() < MAX_STORAGE {
                            storage.push(data);
                            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
                        } else {
                        }
                    },
                    Err(_) => break,
                }
            }
        });
        Handler { sender: tx }
    }
    fn submit(&self, input: String) {
        let _ = self.sender.send(input);
    }
}
pub fn access_count() -> usize {
    GLOBAL_COUNT.load(Ordering::SeqCst)
}
fn main() {
    let handler = Handler::new();
    for _ in 0..150 {
        handler.submit("B".repeat(10));
    }
    thread::sleep(std::time::Duration::from_millis(100));
    println!("Processed count: {}", access_count());
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    // The acceptable maximum number of processed messages in the fixed version.
    const MAX_ACCEPTABLE: usize = 100;

    #[test]
    fn test_resource_control() {
        // Create an instance of the processing component.
        let handler = crate::Handler::new();
        
        // Enqueue 150 messages that are moderately sized.
        for _ in 0..150 {
            handler.submit("Test".repeat(10));
        }
        // Allow sufficient time for the background thread to process the messages.
        thread::sleep(Duration::from_millis(100));
        let count = crate::access_count();
        // In the corrected design, the number of processed messages should not exceed MAX_ACCEPTABLE.
        assert!(
            count <= MAX_ACCEPTABLE,
            "Resource consumption exceeded acceptable limit: {}",
            count
        );
    }
}
