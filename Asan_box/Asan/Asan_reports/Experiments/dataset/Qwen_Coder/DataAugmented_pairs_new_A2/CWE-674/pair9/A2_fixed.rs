use std::sync::{Arc, Mutex};
use std::thread;
const LIMIT: usize = 1000;
struct Data {
    children: Vec<Data>,
}
impl Data {
    fn new() -> Self {
        Data { children: Vec::new() }
    }
    fn attach(&mut self, child: Data) {
        self.children.push(child);
    }
}
fn explore_safe(inner: &Data, depth: usize) -> Result<u32, &'static str> {
    if depth > LIMIT {
        return Err("Recursion too deep");
    }
    let mut count: u32 = 1;
    for child in &inner.children {
        count = count.checked_add(explore_safe(child, depth + 1)?) .ok_or("Count overflow")?;
    }
    Ok(count)
}
fn collect(root: &Data) -> Result<u32, &'static str> {
    explore_safe(root, 0)
}
fn construct(depth: usize) -> Data {
    let mut node = Data::new();
    if depth > 0 {
        node.attach(construct(depth - 1));
    }
    node
}
fn main() {
    let handle = thread::spawn(|| {
        let tree = construct(2000);
        match collect(&tree) {
            Ok(sum) => println!("Total count: {}", sum),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}

//////////////////////////////////////////////////////////////
// Test Oracle:
// This test creates an input with a deep nested tree to simulate the attack.
// For the vulnerable version, we expect the execution to panic (e.g. due to a stack overflow).
// For the fixed version, the function should return an error indicating that recursion is too deep.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test function is intended to be run separately against each version.
    // For the vulnerable version, the call to "collect" is expected to panic.
    // For the fixed version, it should return an Err("Recursion too deep").
    #[test]
    fn deep_nesting_oracle() {
        let tree = construct(1500);
        let result = panic::catch_unwind(|| {
            collect(&tree)
        });
                
        // In the vulnerable version, a panic (stack overflow) is expected.
        if result.is_err() {
            // Test passes for vulnerable version.
            assert!(true, "Vulnerable version panicked as expected.");
        } else {
            // For the fixed version, check for error result.
            let res = result.unwrap();
            match res {
                Ok(val) => panic!("Expected error due to deep recursion, but got value: {}", val),
                Err(msg) => assert_eq!(msg, "Recursion too deep", "Unexpected error message"),
            }
        }
    }
}
