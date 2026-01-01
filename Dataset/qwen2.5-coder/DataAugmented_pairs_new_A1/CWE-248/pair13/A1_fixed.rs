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