use std::fmt;
struct DataHolder {
    data: Vec<u32>,
}
impl DataHolder {
    fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        unsafe {
            vec.set_len(capacity);
        }
        Self { data: vec }
    }
    pub unsafe fn inject(&mut self, count: usize, value: u32) {
        let ptr = self.data.as_mut_ptr();
        self.data.set_len(count);
        for i in 0..count {
            ptr.add(i).write(value);
        }
    }
    pub fn compute(&self) -> u32 {
        self.data.iter().sum()
    }
}
fn run() -> u32 {
    let mut holder = DataHolder::new(10);
    unsafe {
        holder.inject(10, 42);
    }
    holder.compute()
}
fn main() {
    let total = run();
    println!("Sum: {}", total);
}

/////////////////////
// Test Oracle
/////////////////////
#[cfg(test)]
mod tests {
    // The external function 'run' is expected to return the sum of the filled buffer.
    // For a proper fill of 10 elements with the value 42, the expected sum is 420.
    // In the vulnerable implementation, the out-of-bounds write may corrupt memory,
    // causing the computed sum to deviate from this expected value or trigger undefined behavior.
    #[test]
    fn out_of_bounds_oracle() {
        let result = crate::run();
        assert_eq!(result, 420, "Expected sum is 420; vulnerability causes a deviation.");
    }
}
