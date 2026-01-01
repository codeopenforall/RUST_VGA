use std::thread;

struct Manager;

impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, String> {
        let handle = thread::spawn(move || {
            if value == 42 {
                panic!("Unexpected value");
            }
            value * 2
        });

        let res = handle.join().unwrap();
        Ok(res)
    }
}