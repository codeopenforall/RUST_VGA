use std::env;
use std::thread;
const MAX_ALLOCATION: usize = 100; 
struct Processor;
impl Processor {
    pub fn handle(&self, req: usize) -> Result<usize, &'static str> {
        if req > MAX_ALLOCATION {
            return Err("Allocation request exceeds allowed threshold");
        }
        let t = thread::spawn(move || {
            let mut buf = vec![1; req];
            buf.iter().sum()
        });
        match t.join() {
            Ok(sum) => Ok(sum),
            Err(_) => Err("Thread panicked"),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let request: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(50)
    } else {
        50
    };
    let processor = Processor;
    match processor.handle(request) {
        Ok(result) => {
            println!("Result is: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // This test ensures that an allocation request exceeding the threshold is correctly rejected.
    #[test]
    fn test_allocation_limit() {
        let processor = Processor;
        // Using a value that exceeds the allowed maximum.
        let result = processor.handle(101);
        assert!(result.is_err(), "Expected error when allocation request exceeds limit");
    }
}
