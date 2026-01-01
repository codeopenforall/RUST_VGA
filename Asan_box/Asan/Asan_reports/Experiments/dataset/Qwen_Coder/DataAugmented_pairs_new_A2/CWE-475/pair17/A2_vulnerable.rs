#![allow(unused)]
use std::slice;
struct DataHolder {
    pointer: *const u32,
    size: usize,
}
impl DataHolder {
    fn new(data: &[u32]) -> Self {
        DataHolder {
            pointer: data.as_ptr(),
            size: data.len(),
        }
    }
    fn compute(&self) -> u32 {
        unsafe {
            let elements = slice::from_raw_parts(self.pointer, self.size + 1);
            elements.iter().fold(0, |acc, &val| acc + val)
        }
    }
}
fn main() {
    let boxed_data = Box::new([1, 2, 3, 4, 42]);
    let valid_view = &boxed_data[..4];
    let instance = DataHolder::new(valid_view);
    let outcome = instance.compute();
    println!("Total: {}", outcome);
    if outcome == 10 {
        println!("Error: Unexpected valid sum");
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
