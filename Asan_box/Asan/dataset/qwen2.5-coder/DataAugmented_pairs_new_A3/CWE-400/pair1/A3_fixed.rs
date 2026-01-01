struct Engine;

impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        const MAX_LOAD: usize = 100;
        if load > MAX_LOAD {
            return Err("Input load too high".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_control() {
        let eng = Engine;
        // Test with an overloaded input.
        let res = panic::catch_unwind(|| eng.run_input(200));
        match res {
            Ok(val) => {
                // For the fixed version, the function should return an Err.
                assert!(val.is_err(), "Expected an error for input load exceeding the threshold");
                assert_eq!(val.err().unwrap(), "Input load too high");
            },
            Err(_) => {
                // If the function panics, then it is the vulnerable behavior.
                panic!("Test failed: the function panicked instead of returning an error");
            }
        }

        // Test with a normal input.
        let res_normal = eng.run_input(10);
        assert!(res_normal.is_ok(), "Expected normal processing for input load within the limit");
    }
}