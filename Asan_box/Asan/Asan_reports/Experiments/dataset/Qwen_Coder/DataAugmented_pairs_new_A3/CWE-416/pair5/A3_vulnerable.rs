struct Data {
    number: i32,
}

fn acquire_ptr() -> *const i32 {
    let boxed = Box::new(Data { number: 1337 });
    let raw = &boxed.number as *const i32;
    drop(boxed); 
    let _dummy = Box::new(Data { number: 4242 });
    raw
}

fn unsafe_compute(ptr: *const i32) -> i32 {
    unsafe {
        *ptr
    }
}

pub fn get_result() -> i32 {
    let ptr = acquire_ptr();
    unsafe_compute(ptr)
}

#[cfg(test)]
mod tests {
    // Import the public function from the code under test.
    // For the vulnerable build, this test is expected to fail; for the fixed build, it will pass.
    use super::get_result;

    #[test]
    fn test_oracle() {
        let res = get_result();
        // The contract expects the computed value to be exactly 1337.
        // In the vulnerable version, use‐after‐free may lead to an incorrect value or undefined behavior.
        assert_eq!(res, 1337, "Oracle: Expected 1337 but got {}", res);
    }
}
