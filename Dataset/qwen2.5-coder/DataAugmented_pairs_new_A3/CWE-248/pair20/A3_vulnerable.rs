struct Runner {
    num_ptr: *const i32,
}

impl Runner {
    fn execute(&self) {
        let value = unsafe { *self.num_ptr }; 
        panic!("Thread encountered an unrecovered panic");
    }
}

pub fn run_logic() {
    let runner = Runner { num_ptr: &10 };
    runner.execute();
    println!("Execution completed (this line is never reached in failure cases)");
}