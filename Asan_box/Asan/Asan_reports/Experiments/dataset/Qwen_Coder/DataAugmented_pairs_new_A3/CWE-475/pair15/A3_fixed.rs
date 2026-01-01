use std::alloc::{alloc, dealloc, Layout};

struct DataProcessor;

impl DataProcessor {
    pub fn execute(&self, requested: usize) -> Result<u8, &'static str> {
        let source = vec![42u8; 10];
        if requested > source.len() {
            return Err("Requested size exceeds source length");
        }
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let dest = unsafe { alloc(layout) };
        if dest.is_null() {
            return Err("Allocation failed");
        }
        unsafe {
            std::ptr::copy_nonoverlapping(source.as_ptr(), dest, requested);
        }
        let value = unsafe { *dest };
        unsafe { dealloc(dest, layout) };
        Ok(value)
    }
}

pub fn process_input(requested: usize) -> Result<u8, &'static str> {
    let processor = DataProcessor;
    processor.execute(requested)
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
