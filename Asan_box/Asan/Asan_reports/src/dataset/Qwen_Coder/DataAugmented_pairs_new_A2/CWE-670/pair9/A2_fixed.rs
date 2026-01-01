use std::sync::Arc;
use std::thread;
type Operation = fn(i32) -> i32;
fn sensitive(val: i32) -> i32 {
    val * 10
}
fn nonsensitive(val: i32) -> i32 {
    val + 5
}
struct Dispatcher {
    table: [Operation; 2],
    safe_mode: bool,
}
impl Dispatcher {
    fn new(safe_mode: bool) -> Self {
        Dispatcher {
            table: [nonsensitive, sensitive],
            safe_mode,
        }
    }
    fn process(&self, input: i32) -> i32 {
        if self.safe_mode {
            self.table[0](input)
        } else {
            let index = if input % 2 == 0 { 0 } else { 1 };
            self.table[index](input)
        }
    }
}
fn main() {
    let dispatcher = Arc::new(Dispatcher::new(true));
    let mut handles = vec![];
    for _ in 0..4 {
        let disp = Arc::clone(&dispatcher);
        handles.push(thread::spawn(move || {
            let result = disp.process(3);
            println!("Outcome: {}", result);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // A test oracle that ensures the correct branch is executed.
    // In safe mode, the outcome must always be computed by nonsensitive(), i.e., input + 5.
    #[test]
    fn test_control_flow() {
        // Create an instance with safe_mode activated.
        let dsp = Dispatcher::new(true);
        // Test input chosen to trigger the flawed branch in the vulnerable version (odd number).
        let test_input = 3;
        let outcome = dsp.process(test_input);
        // Expected outcome: nonsensitive(3) = 3 + 5 = 8.
        // In the vulnerable version, outcome would be sensitive(3) = 30.
        assert_eq!(outcome, 8, "The operation branch is incorrectly selected based on untrusted input.");
    }
}
