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