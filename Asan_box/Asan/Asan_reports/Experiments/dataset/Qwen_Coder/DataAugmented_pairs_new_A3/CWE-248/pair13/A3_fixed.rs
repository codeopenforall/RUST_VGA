use std::thread;

struct Processor {
    callback: fn(&str) -> Option<String>,
}

impl Processor {
    pub fn execute(&self, input: &str) -> Option<String> {
        (self.callback)(input)
    }
}

fn callback_fn(msg: &str) -> Option<String> {
    if msg == "trigger" {
        eprintln!("Simulated panic occurred!");
        None
    } else {
        Some(format!("Processed: {}", msg))
    }
}

fn main() {
    let proc = Processor { callback: callback_fn };
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
