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