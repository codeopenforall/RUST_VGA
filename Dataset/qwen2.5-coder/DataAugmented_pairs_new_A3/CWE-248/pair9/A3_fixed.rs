pub struct Controller;

impl Controller {
    pub fn run(&self, value: i32) -> Result<i32, &'static str> {
        use std::panic;

        let res = panic::catch_unwind(|| {
            unsafe {
                if value == 0 {
                    Err("Division by zero")
                } else {
                    Ok(100 / value)
                }
            }
        });

        match res {
            Ok(inner) => inner,
            Err(_) => Err("Thread panicked"),
        }
    }
}