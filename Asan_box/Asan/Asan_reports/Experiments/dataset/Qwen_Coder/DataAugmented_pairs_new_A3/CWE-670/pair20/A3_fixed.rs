use std::sync::{Arc, Mutex};
use std::thread;

struct TaskManager {
    tasks: Vec<i32>,
}

impl TaskManager {
    fn new(size: usize) -> Self {
        TaskManager { tasks: vec![0; size] }
    }

    fn execute(&mut self, idx: usize, value: i32, perform: bool) -> Result<(), &'static str> {
        match perform {
            true => {
                if idx < self.tasks.len() {
                    self.tasks[idx] = value;
                    Ok(())
                } else {
                    Err("index out of bounds")
                }
            }
            false => {
                if idx < self.tasks.len() {
                    self.tasks[idx] = value;
                    Ok(())
                } else {
                    Err("index out of bounds")
                }
            }
        }
    }

    fn get_tasks(&self) -> &[i32] {
        &self.tasks
    }
}

fn main() {
    let mgr = Arc::new(Mutex::new(TaskManager::new(3)));
    let mut handles = vec![];
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            mgr.execute(0, 100, false).unwrap();
        }));
    }
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            mgr.execute(0, 200, true).unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mgr = mgr.lock().unwrap();
    println!("Tasks: {:?}", mgr.get_tasks());
}

//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_behavior() {
        // Initialize TaskManager with a known state.
        let mut tm = TaskManager::new(3);
        
        // When the flag is false, no update should occur.
        tm.execute(0, 100, false).expect("Execution should succeed");
        // Immediately update with flag true: should update index 0 only.
        tm.execute(0, 200, true).expect("Execution should succeed");
        
        // For the corrected code, the expected state is [200, 0, 0].
        // For the vulnerable version, the state would be [100, 200, 0] causing the test to fail.
        assert_eq!(tm.get_tasks(), &[200, 0, 0], "The TaskManager state did not match the expected invariant");
    }
}
