pub struct Engine;

impl Engine {
    pub fn run(&self, trigger: bool) -> i32 {
        if trigger {
            unsafe {
                panic!("uncaught panic in unsafe block");
            }
        }
        0
    }
}