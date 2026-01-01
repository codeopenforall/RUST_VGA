use std::thread;

struct Runner {
    num_ptr: *const i32,
}

impl Runner {
    fn execute(&self) -> Result<(), ()> {
        unsafe {
            let value = *self.num_ptr;
            println!("Value: {}", value);
        }
        let handle = thread::spawn(|| {
            panic!("Thread panic captured safely");
        });

        match handle.join() {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Recovered from thread panic: {:?}", err);
                Ok(())
            }
        }
    }
}

pub fn run_logic() -> Result<(), ()> {
    let runner = Runner {
        num_ptr: &42,
    };
    runner.execute()?;
    println!("Execution completed safely");
    Ok(())
}