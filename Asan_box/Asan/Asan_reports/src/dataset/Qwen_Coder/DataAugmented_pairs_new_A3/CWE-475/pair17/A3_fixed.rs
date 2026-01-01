#![allow(unused)]
use std::slice;
struct Container {
    ptr: *const u32,
    len: usize,
}
impl Container {
    fn new(input: &[u32]) -> Self {
        Container {
            ptr: input.as_ptr(),
            len: input.len(),
        }
    }
    fn aggregate(&self) -> u32 {
        unsafe {
            let data = slice::from_raw_parts(self.ptr, self.len);
            data.iter().fold(0, |acc, &val| acc + val)
        }
    }
}
fn main() {
    let boxed = Box::new([1, 2, 3, 4, 42]);
    let valid_slice = &boxed[..4];
    let handler = Container::new(valid_slice);
    let result = handler.aggregate();
    println!("Total: {}", result);
    if result != 10 {
        println!("Error: Incorrect sum computed");
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn integration_oracle() {
        // This test assumes that the compiled binary is named "demo"
        // and located in the current directory.
        // For the vulnerable code, the printed total will be 52, while for the fixed version, it will be 10.
        let output = Command::new("./demo")
            .output()
            .expect("Failed to execute binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        // The oracle expects the fixed output ("Total: 10"). If not found, the test fails.
        assert!(
            stdout.contains("Total: 10"),
            "Oracle failure: expected 'Total: 10', got: {}",
            stdout
        );
    }
}
