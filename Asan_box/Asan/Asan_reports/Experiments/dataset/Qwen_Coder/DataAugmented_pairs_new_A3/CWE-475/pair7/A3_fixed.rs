struct MyStruct {
    elems: Vec<i32>,
}

impl MyStruct {
    pub fn new(elems: Vec<i32>) -> Self {
        MyStruct { elems }
    }

    pub fn get(&self, pos: usize) -> i32 {
        if pos < self.elems.len() {
            self.elems[pos]
        } else {
            panic!("Provided index {} is out-of-bound (max index {}).", pos, self.elems.len() - 1);
        }
    }
}

pub fn execute(index: usize) {
    let my_struct = MyStruct::new(vec![1, 2, 3, 4, 5]);
    my_struct.get(index);
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Import the appropriate run function based on whether we're testing the vulnerable or fixed variant.
    // Assume that in the test build, the function 'execute' is available in the parent module.
    // The test triggers the out-of-bound scenario with a predetermined index that should not be valid.
    #[test]
    fn boundary_check_test() {
        // In both variants, an out-of-bound index (e.g., 10) is used.
        // For the fixed variant, the bounds check causes a panic,
        // which is the intended and correct behavior.
        // In the vulnerable variant, undefined behavior might lead to a regular return value
        // or even a silent memory corruption, so the test will fail if no panic occurs.
        let result = panic::catch_unwind(|| {
            super::execute(10);
        });
        assert!(result.is_err(), "Expected a panic due to out-of-bound access, but no panic occurred.");
    }
}
