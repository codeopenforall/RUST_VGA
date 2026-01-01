use std::env;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::ffi::c_void;
struct Executor;
impl Executor {
    pub fn run(input: &str) -> Result<(), String> {
        let shared_counter = Arc::new(Mutex::new(0));
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];
        for i in 0..4 {
            let counter = Arc::clone(&shared_counter);
            let param = input.to_owned();
            let thread_tx = tx.clone();
            let handle = thread::spawn(move || {
                let op_result = unsafe { risky_operation(i, &param) };
                if let Err(e) = op_result {
                    let _ = thread_tx.send(Err(e.to_owned()));
                    return;
                } else {
                    let _ = thread_tx.send(Ok(()));
                }
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            threads.push(handle);
        }
        drop(tx);
        for result in rx {
            if let Err(msg) = result {
                for th in threads {
                    let _ = th.join();
                }
                return Err(format!("A task failed: {}", msg));
            }
        }
        for th in threads {
            let _ = th.join();
        }
        if *shared_counter.lock().unwrap() == 4 {
            Ok(())
        } else {
            Err("Not all tasks completed correctly".to_owned())
        }
    }
}
unsafe fn risky_operation(task_id: u32, param: &str) -> Result<(), &'static str> {
    if param.contains("fail") && task_id == 2 {
        Err("Operation failed in unsafe block")
    } else {
        Ok(())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "pass".to_owned()
    };
    let _ = Executor::run(&input);
    println!("Execution finished.");
}

--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::str;

    // This test function uses the command line interface of the target binary.
    // It executes the built binary with the parameter "fail" which should trigger the intended failure.
    // The test expects that:
    // - The vulnerable code execution would ignore the error and report "Execution finished."
    // - The corrected fixed code would report an error message indicating that a task failed.
    //
    // For the purpose of this test oracle, assume that the binary built from the fixed code
    // will output a line starting with "Execution encountered an error:" when given input "fail",
    // while the vulnerable binary will output "Execution finished." (thus failing the test).
    //
    // The test passes if "fail" input results in error detection.
    #[test]
    fn test_error_detection() {
        // Change the executable name accordingly when running tests.
        // For demonstration, we assume the binary is available as "./app"
        // Adjust the path if necessary.
        let output = Command::new("./app")
            .arg("fail")
            .output()
            .expect("Failed to execute process");

        let stdout = str::from_utf8(&output.stdout).unwrap();

        // In the corrected (fixed) code, an error is correctly reported for the "fail" input.
        // The test asserts that the output should contain the error message.
        assert!(stdout.contains("Execution encountered an error:"), "Expected error not detected. Output was: {}", stdout);
    }
}
--------------------------------------------------
