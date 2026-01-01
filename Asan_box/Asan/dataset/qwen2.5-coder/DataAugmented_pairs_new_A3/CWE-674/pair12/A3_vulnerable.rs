use std::panic;

pub struct Engine;

impl Engine {
    pub unsafe fn explore(&self, data: *const u8, len: usize, idx: usize) -> i32 {
        if idx >= len {
            return 0;
        }
        return 1 + self.explore(data, len, idx + 1);
    }

    pub fn journey(&self, input: &[u8]) -> i32 {
        unsafe { self.explore(input.as_ptr(), input.len(), 0) }
    }
}

pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec();
    let res = engine.journey(&inp);
    Ok(res)
}

fn main() {
    let deep_input = "(".repeat(5000);
    match evaluate(&deep_input) {
        Ok(res) => println!("Computation successful, result: {}", res),
        Err(err) => println!("Computation failed with error: {}", err),
    }
}