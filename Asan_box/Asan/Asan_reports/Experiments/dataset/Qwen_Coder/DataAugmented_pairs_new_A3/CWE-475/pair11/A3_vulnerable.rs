use std::io::{self, Write};

struct Data {
    values: Vec<i32>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        Data {
            values: vec![0; size],
        }
    }

    pub fn write(&mut self, index: usize, value: i32) -> io::Result<()> {
        self.values[index] = value;
        Ok(())
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}

pub fn run_harness() -> io::Result<i32> {
    let mut data = Data::new(5);
    data.write(4, 42); // Intentionally out-of-bounds write
    let s = data.sum();
    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::run_harness;
    #[test]
    fn test_api_precondition() {
        // The contract is that when violating the API preconditions,
        // the fixed version detects the issue (returning an error)
        // while the vulnerable version erroneously returns an Ok result.
        let result = run_harness();
        assert!(
            result.is_err(),
            "Expected an error due to out-of-bound access, but got: {:?}",
            result
        );
    }
}
