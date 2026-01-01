use std::sync::Arc;
use std::thread;

struct Computation {
    numerator: i32,
    denominator: i32,
}

impl Computation {
    fn execute(&self) -> Result<i32, String> {
        if self.denominator == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(self.numerator / self.denominator)
        }
    }
}

fn main() {
    let instance = Arc::new(Computation { numerator: 100, denominator: 0 });
    let instance_clone = Arc::clone(&instance);
    let handler = thread::spawn(move || {
        instance_clone.execute()
    });
    match handler.join().expect("Thread panicked") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}