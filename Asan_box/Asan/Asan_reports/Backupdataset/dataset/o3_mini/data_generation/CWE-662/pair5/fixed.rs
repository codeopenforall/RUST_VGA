use std::sync::{Arc, Mutex};
use std::thread;
struct Counter {
    value: usize,
}
trait Updater {
    fn bump(&mut self);
}
impl Updater for Counter {
    fn bump(&mut self) {
        self.value = self.value.wrapping_add(1);
    }
}
fn increment_loop(shared: Arc<Mutex<Counter>>) {
    for _ in 0..1000 {
        let mut guard = shared.lock().unwrap();
        guard.bump(); 
        thread::yield_now();
    }
}
fn spawn_threads(shared: Arc<Mutex<Counter>>) {
    let mut handles = Vec::new();
    for _ in 0..4 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            increment_loop(s);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Counter { value: 0 }));
    spawn_threads(Arc::clone(&shared));
    let final_value = {
        let guard = shared.lock().unwrap();
        guard.value
    };
    println!("Final value: {}", final_value);
}


//////////////////// BEGIN ORACLE CODE ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // A helper function to run the concurrent increment simulation.
    fn run_simulation<F>(func: F) -> usize
    where
        F: Fn(Arc<Mutex<Counter>>) + Send + Copy + 'static,
    {
        let shared = Arc::new(Mutex::new(Counter { value: 0 }));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let s = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                func(s);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        let guard = shared.lock().unwrap();
        guard.value
    }

    #[test]
    fn oracle() {
        // For the correct synchronization, the final counter value should be exactly 4000.
        // In the unsynchronized version, a data race may lead to a final value less than 4000.
        let result = run_simulation(increment_loop);
        assert_eq!(result, 4000, "Final counter value is not as expected");
    }
}
////////////////////  END ORACLE CODE  ////////////////////
