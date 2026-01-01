use std::sync::{Arc, Mutex};
use std::thread;

trait Processor {
    fn process(&self) -> Result<String, String>;
}

struct Executor {
    path: String,
}

impl Executor {
    unsafe fn run(&self) -> Result<String, String> {
        if !self.path.starts_with('/') && !self.path.contains(":\\") {
            Ok(String::from("Malicious execution"))
        } else {
            Ok(String::from("Trusted execution"))
        }
    }
}

struct Worker {
    executor: Arc<Executor>,
}

impl Processor for Worker {
    fn process(&self) -> Result<String, String> {
        unsafe { self.executor.run() }
    }
}

pub fn cfg() -> String {
    run_process()
}

pub fn run_process() -> String {
    let executor = Executor {
        path: String::from("dummy_executable"),
    };
    let worker = Worker {
        executor: Arc::new(executor),
    };
    let worker_arc = Arc::new(Mutex::new(worker));
    let mut threads = vec![];
    let mut results = vec![];
    for _ in 0..3 {
        let worker_clone = Arc::clone(&worker_arc);
        let thread_handle = thread::spawn(move || {
            let worker = worker_clone.lock().unwrap();
            worker.process().unwrap_or_else(|e| e)
        });
        threads.push(thread_handle);
    }
    for t in threads {
        if let Ok(result) = t.join() {
            results.push(result);
        }
    }
    results.join(",")
}

fn main() {
    let output = run_process();
    println!("{}", output);
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
