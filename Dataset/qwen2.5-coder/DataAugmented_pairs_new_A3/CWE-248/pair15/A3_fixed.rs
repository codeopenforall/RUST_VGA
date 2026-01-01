pub struct Engine;

impl Engine {
    pub fn run(&self, trigger: bool) -> i32 {
        if trigger {
            let handle = std::thread::spawn(|| {
                unsafe {
                    // Simulate an unsafe block that might cause a panic
                    panic!("panic in unsafe block caught later");
                }
            });

            match handle.join() {
                Ok(val) => val,
                Err(_) => -1,
            }
        } else {
            0
        }
    }
}