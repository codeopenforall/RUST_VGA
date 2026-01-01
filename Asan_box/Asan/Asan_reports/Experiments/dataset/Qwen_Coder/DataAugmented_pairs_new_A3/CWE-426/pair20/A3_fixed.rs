use std::sync::{Arc, Mutex};
use std::thread;

trait Processor {
    fn process(&self) -> Result<String, String>;
}

struct Loader {
    path: String,
}

impl Loader {
    fn execute(&self) -> Result<String, String> {
        if self.path.starts_with('/') || self.path.contains(":\\") {
            Ok(String::from("Trusted execution"))
        } else {
            Ok(String::from("Malicious execution"))
        }
    }
}

struct Agent {
    loader: Arc<Loader>,
}

impl Processor for Agent {
    fn process(&self) -> Result<String, String> {
        self.loader.execute()
    }
}

pub fn run_process() -> String {
    let loader = Loader {
        path: String::from("/safe_executable"),
    };
    let agent = Agent {
        loader: Arc::new(loader),
    };
    let agent_arc = Arc::new(Mutex::new(agent));
    let mut handles = vec![];
    let mut outputs = vec![];
    for _ in 0..3 {
        let agent_clone = Arc::clone(&agent_arc);
        let handle = thread::spawn(move || {
            let agent = agent_clone.lock().unwrap();
            agent.process().unwrap_or_else(|e| e)
        });
        handles.push(handle);
    }
    for h in handles {
        if let Ok(val) = h.join() {
            outputs.push(val);
        }
    }
    outputs.join(",")
}

fn main() {
    let result = run_process();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::run_process;

    #[test]
    fn test_execution() {
        // The expected safe execution output should be three instances of "Trusted execution" joined by commas.
        let expected = "Trusted execution,Trusted execution,Trusted execution";
        let result = run_process();
        // For the vulnerable version, run_process will return "Malicious execution,Malicious execution,Malicious execution"
        // causing this test to fail; for the fixed version, the test will pass.
        assert_eq!(result, expected, "Execution output does not match expected trusted result");
    }
}
