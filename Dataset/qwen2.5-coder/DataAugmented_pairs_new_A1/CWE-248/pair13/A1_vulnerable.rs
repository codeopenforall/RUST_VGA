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