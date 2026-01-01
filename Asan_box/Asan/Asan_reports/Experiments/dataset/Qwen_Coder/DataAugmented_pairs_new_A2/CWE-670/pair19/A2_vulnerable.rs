use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Level {
    High,
    Low,
}

trait Compute {
    fn run(&self) -> i32;
}

struct Task {
    num: i32,
    lev: Level,
}

impl Compute for Task {
    fn run(&self) -> i32 {
        unsafe {
            let ptr = &self.num as *const i32;
            let value = *ptr;
            match self.lev {
                Level::High => {
                    return value + 5;
                }
                Level::Low => {
                    return value.wrapping_mul(2);
                }
            }
        }
    }
}

pub fn execute_tasks() -> Vec<i32> {
    let tasks = Arc::new(Mutex::new(vec![
        Task { num: 20, lev: Level::High },
        Task { num: 10, lev: Level::Low },
    ]));
    let mut handles = vec![];
    for _ in 0..2 {
        let tasks_cloned = Arc::clone(&tasks);
        let handle = thread::spawn(move || {
            let mut results = Vec::new();
            let jobs = tasks_cloned.lock().unwrap();
            for job in jobs.iter() {
                results.push(job.run());
            }
            results
        });
        handles.push(handle);
    }
    let mut final_results = Vec::new();
    for handle in handles {
        let thread_results = handle.join().unwrap();
        final_results.extend(thread_results);
    }
    // Introduce a defect: reverse the results before returning
    final_results.reverse();
    final_results
}

fn main() {
    let output = execute_tasks();
    println!("Output: {:?}", output);
}

///////////////////////////////////////////////
// Test Oracle for verifying correct behavior
//
// This test calls the public execute_tasks() helper, sorts the resulting
// vector, and asserts that the fixed logic produces the expected outputs.
// Expected outcomes for tasks with input 20 (High) and 10 (Low) are 40 and 15, respectively,
// repeated twice (due to two threads). The test will fail when run against the vulnerable version.
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_tasks;

    #[test]
    fn verify_computation() {
        let mut results = execute_tasks();
        results.sort();
        let mut expected = vec![40, 15, 40, 15];
        expected.sort();
        assert_eq!(results, expected, "The computation did not yield the expected results.");
    }
}
