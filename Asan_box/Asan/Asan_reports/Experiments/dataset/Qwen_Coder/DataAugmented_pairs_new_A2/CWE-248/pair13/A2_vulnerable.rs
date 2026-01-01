use std::thread;

struct Processor {
    handler: fn(&str) -> Result<String, String>,
}

impl Processor {
    pub fn execute(&self, input: &str) -> Option<String> {
        match (self.handler)(input) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

fn handler_fn(msg: &str) -> Result<String, String> {
    if msg == "trigger" {
        return Err("Simulated error occurred!".to_string());
    }
    Ok(format!("Processed: {}", msg))
}

fn main() {
    let proc = Processor { handler: handler_fn };
    let handle = thread::spawn(move || {
        let _ = proc.execute("trigger");
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_exception_handling() {
        let proc = Processor { callback: callback_fn };
        // The execution should not propagate a panic.
        // In the fixed version, execute("trigger") returns None.
        let result = panic::catch_unwind(|| proc.execute("trigger"));
        assert!(result.is_ok(), "Execution panicked unexpectedly");
        let output = result.unwrap();
        assert!(output.is_none(), "Expected None when panic occurs");
    }
}
