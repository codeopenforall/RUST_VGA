use std::thread;
const MAX_ITEMS: usize = 1_000_000;
struct ResourceManager;
impl ResourceManager {
    fn new() -> Self {
        ResourceManager
    }
    fn allocate_items(&self, count: usize) -> Result<Vec<u8>, &'static str> {
         // Remove the check for count > MAX_ITEMS
         let vec = vec![0xAA; count];
         Ok(vec)
    }
}
fn process(count: usize) -> Result<Vec<u8>, &'static str> {
    let manager = ResourceManager::new();
    let handle = thread::spawn(move || {
         manager.allocate_items(count)
    });
    handle.join().unwrap()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
         args[1].parse().unwrap_or(0)
    } else {
         0
    };
    match process(count) {
         Ok(vec) => println!("Allocation completed, size: {}", vec.len()),
         Err(e) => println!("Failed: {}", e),
    }
}

/*
This is the test oracle function. It defines a unit test that invokes the process() function 
with an allocation size just above the allowed limit. The test expects the corrected (fixed) version 
to return an error ("Requested allocation exceeds limit"), whereas the vulnerable version 
would incorrectly proceed with the allocation and return Ok(_), triggering the test failure.
*/
#[cfg(test)]
mod tests {
    use super::*;
    const LIMIT: usize = 1_000_000;
    
    #[test]
    fn test_resource_throttling() {
         // Use an input value one greater than the allowed threshold.
         let huge = LIMIT + 1;
         let result = process(huge);
         match result {
              Ok(v) => panic!("Test failed: allocation was not throttled. Allocated {} items.", v.len()),
              Err(e) => assert_eq!(e, "Requested allocation exceeds limit"),
         }
    }
}
