use std::vec::Vec;

struct TaskManager {
    tasks: Vec<i32>,
}

impl TaskManager {
    pub fn new(size: usize) -> Self {
        TaskManager {
            tasks: vec![0; size],
        }
    }

    pub fn execute(&mut self, idx: usize, value: i32, perform: bool) -> Result<(), &'static str> {
        unsafe {
            match perform {
                true => {
                    if idx < self.tasks.len() - 1 {
                        let ptr = self.tasks.as_mut_ptr().add(idx + 1);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
                false => {
                    if idx < self.tasks.len() {
                        let ptr = self.tasks.as_mut_ptr().add(idx);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
            }
        }
    }

    pub fn get_tasks(&self) -> &[i32] {
        &self.tasks
    }
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
