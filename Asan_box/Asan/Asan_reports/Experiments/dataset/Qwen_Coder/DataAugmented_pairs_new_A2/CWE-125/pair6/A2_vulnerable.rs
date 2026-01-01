use std::env;
use std::process;

trait AccessItem {
    fn fetch(&self, idx: usize) -> Result<u32, &'static str>;
}

struct DataStore {
    elements: &'static [u32],
}

impl AccessItem for DataStore {
    fn fetch(&self, idx: usize) -> Result<u32, &'static str> {
        unsafe {
            let element = *self.elements.get_unchecked(idx);
            Ok(element)
        }
    }
}

fn initialize_store() -> DataStore {
    static COMPLETE: [u32; 4] = [10, 20, 30, 0];
    let segment = &COMPLETE[..3];
    DataStore { elements: segment }
}

fn execute(idx: usize) -> Result<u32, &'static str> {
    let store = initialize_store();
    store.fetch(idx)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("Usage: cargo run <index>");
        process::exit(1);
    }
    let idx: usize = arguments[1].parse().unwrap_or(0);
    match execute(idx) {
        Ok(val) => println!("Value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_out_of_bounds() {
        // For a proper implementation, an index equal to the slice length should return an error.
        //
        // In the unsafe version, run(3) may return a spurious value (or invoke undefined behavior),
        // failing this contract.
        let result = run(3);
        assert!(result.is_err(), "Expected an error for out-of-bounds access (index 3), but got: {:?}", result);
    }
}
