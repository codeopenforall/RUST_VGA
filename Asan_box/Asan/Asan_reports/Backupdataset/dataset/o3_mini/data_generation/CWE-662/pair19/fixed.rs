use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Data {
    part1: Mutex<u32>,
    part2: Mutex<u32>,
}
impl Data {
    fn new() -> Self {
        Data {
            part1: Mutex::new(0),
            part2: Mutex::new(0),
        }
    }
}
unsafe fn compute(arc: &Arc<Data>) -> u32 {
    let arc_clone = Arc::clone(arc);
    let t1 = thread::spawn(move || {
        unsafe {
            let mut p1 = arc_clone.part1.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let mut p2 = arc_clone.part2.lock().unwrap();
            *p1 += 1;
            *p2 += 2;
        }
    });
    let arc_clone = Arc::clone(arc);
    let t2 = thread::spawn(move || {
        unsafe {
            let mut p1 = arc_clone.part1.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let mut p2 = arc_clone.part2.lock().unwrap();
            *p2 += 3;
            *p1 += 4;
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let r1 = *arc.part1.lock().unwrap();
    let r2 = *arc.part2.lock().unwrap();
    r1 + r2
}
fn main() {
    let data = Arc::new(Data::new());
    let result = unsafe { compute(&data) };
    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc};
    use std::thread;
    use std::time::Duration;
    // Assume that Data and compute are available in the crate's root.

    fn run_compute() -> u32 {
        let data = Arc::new(crate::Data::new());
        // Safety: The compute function is marked unsafe.
        unsafe { crate::compute(&data) }
    }

    #[test]
    fn test_deadlock() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let res = run_compute();
            tx.send(res).unwrap();
        });
        // The expected computed sum is: (1+4)+ (2+3) = 10.
        // If a deadlock occurs (vulnerable case), the channel will timeout.
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(result) => assert_eq!(result, 10, "Expected computed sum to be 10"),
            Err(_) => panic!("Test timed out, indicating a potential deadlock"),
        }
    }
}
