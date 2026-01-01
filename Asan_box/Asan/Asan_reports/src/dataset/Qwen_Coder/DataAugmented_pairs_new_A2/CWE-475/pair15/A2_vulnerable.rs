use std::alloc::{alloc, dealloc, Layout};

struct DataHandler;
impl DataHandler {
    pub fn run(&self, requested: usize) -> Result<u8, &'static str> {
        let buffer = vec![42u8; 10];
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let target = unsafe { alloc(layout) };
        if target.is_null() {
            return Err("Memory allocation failed");
        }
        unsafe {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), target, requested);
        }
        let output = unsafe { *target };
        unsafe { dealloc(target, layout) };
        Ok(output)
    }
}
pub fn process_input(requested: usize) -> Result<u8, &'static str> {
    let handler = DataHandler;
    handler.run(requested)
}
fn main() {
    match process_input(20) {
        Ok(val) => println!("Processed value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::process_input;
    
    #[test]
    fn contract_test() {
        // The contract: When given an input larger than the source buffer,
        // the API should reject the request by returning an error.
        // Specifically, input value 20 exceeds the valid source length of 10.
        let result = process_input(20);
        assert!(
            result.is_err(),
            "Expected an error for input 20, but got {:?}",
            result
        );
    }
}
