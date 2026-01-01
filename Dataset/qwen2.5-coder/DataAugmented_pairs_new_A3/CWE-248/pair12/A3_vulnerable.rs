struct HandlerThread {
    raw: *const i32,
}

impl HandlerThread {
    fn run(&self, value: i32) -> i32 {
        unsafe {
            let result = *self.raw;
            if result < 0 {
                result
            } else {
                value
            }
        }
    }
}

fn execute_task(value: i32) -> i32 {
    let handler_thread = HandlerThread { raw: &-1 };
    let res = handler_thread.run(value);
    res
}

fn main() {
    let output = execute_task(-1);
    println!("Result: {}", output);
}