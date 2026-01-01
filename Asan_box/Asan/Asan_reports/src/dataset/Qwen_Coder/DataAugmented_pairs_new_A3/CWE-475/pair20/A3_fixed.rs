pub struct Container {
    pub payload: Vec<u8>,
}

impl Container {
    pub fn compute(&self) -> Option<i32> {
        if self.payload.len() < 12 {
            return None;
        }

        let data: Vec<u8> = vec![
            1, 0, 0, 0,   
            2, 0, 0, 0,   
            3, 0, 0, 0    
        ];

        let sum: i32 = data.iter().map(|&x| x as i32).sum();
        Some(sum)
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    // Import the Container from the current crate.
    use crate::Container;

    // This test serves as the oracle for the contract.
    // It uses an input that is intentionally too short (8 bytes) so that:
    // - In the corrected version, the compute method returns None.
    // - In the vulnerable version, the lack of precondition checking leads to undefined behavior,
    //   typically resulting in an incorrect (non-None) value or even a panic.
    #[test]
    fn test_container() {
        let container = Container { payload: vec![1u8, 2, 3, 4, 5, 6, 7, 8] };

        let handle = thread::spawn(move || {
            container.compute()
        });

        let result = handle.join().expect("Thread panicked");
        // The contract expects that an invalid input returns None.
        assert_eq!(result, None, "Expected None due to invalid input; computation did not fail as expected");
    }
}
