struct App {
    data: Vec<i32>,
}

impl App {
    pub fn new(flag: bool) -> Self {
        if flag {
            App { data: vec![10, 20] }
        } else {
            App { data: vec![30, 40] }
        }
    }

    pub fn process(&self, idx: usize) -> Result<i32, &'static str> {
        self.data.get(idx).copied().ok_or("Index out of bounds")
    }
}

/*
   Test oracle function for verifying the vulnerability and its fix.
   This test should be compiled and run separately from both the vulnerable and fixed code.
   It creates an instance of App with flag true and calls process(0). For the fixed version, 
   the returned value must be 10. For the vulnerable version, due to the incorrect control flow, 
   the value will be 20. The assertion below is expected to fail against the vulnerable implementation 
   and pass against the fixed implementation.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_process_output() {
        let app = Arc::new(App::new(true));
        let app_clone = Arc::clone(&app);
        let handle = thread::spawn(move || {
            app_clone.process(0)
        });
        let result = handle.join().unwrap();
        // The expected correct value is 10.
        assert_eq!(result.unwrap(), 10, "The process function did not return the expected value");
    }
}
