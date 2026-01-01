use std::sync::Arc;
use std::thread;
struct Data {
    arr: Vec<u8>,
}
impl Data {
    fn new() -> Self {
        Data { arr: vec![10, 20, 30, 40, 50] }
    }
    fn get_at(&self, pos: usize) -> Result<u8, &'static str> {
        if pos < 5 {
            return Err("Input value too small, causes underflow");
        }
        let index = pos - 5; 
        if index >= self.arr.len() {
            return Err("Index out of bounds");
        }
        Ok(self.arr[index])
    }
}
struct Engine {
    data: Arc<Data>,
}
impl Engine {
    fn new(data: Arc<Data>) -> Self {
        Engine { data }
    }
    fn execute(&self, input: usize) -> Result<u8, &'static str> {
        self.data.get_at(input)
    }
}
fn main() {
    let data = Arc::new(Data::new());
    let engine = Engine::new(data.clone());
    let handles: Vec<_> = (5..8).map(|i| {
        let eng = Engine { data: data.clone() };
        thread::spawn(move || {
            let res = eng.execute(i);
            match res {
                Ok(val) => println!("Thread {} result: {}", i, val),
                Err(e) => println!("Thread {} error: {}", i, e),
            }
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // The test oracle expects that using an input that would cause underflow (i.e., less than 5)
    // should result in an error. In the safe implementation, the error is returned.
    // In the vulnerable implementation, the unchecked arithmetic may lead to an unexpected result or a panic.
    #[test]
    fn test_no_underflow() {
        let data = Arc::new(Data::new());
        let engine = Engine::new(data);
        // Using an input that is too small; fixed version should return an error.
        match engine.execute(3) {
            Ok(val) => {
                panic!("Test failed: Expected error for input causing underflow, but got value {}", val);
            },
            Err(msg) => {
                assert_eq!(msg, "Input value too small, causes underflow", "Unexpected error message");
            },
        }
    }
}
