use std::sync::Arc;

struct Data {
    number: i32,
}

fn acquire_shared() -> Arc<Data> {
    Arc::new(Data { number: 1337 })
}

fn safe_compute(shared: &Data) -> i32 {
    shared.number
}

pub fn get_result() -> i32 {
    let shared = acquire_shared();
    safe_compute(&shared)
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
