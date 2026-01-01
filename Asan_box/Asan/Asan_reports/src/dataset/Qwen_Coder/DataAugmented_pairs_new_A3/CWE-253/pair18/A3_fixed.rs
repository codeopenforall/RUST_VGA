struct Processor {
    value: i32,
}

impl Processor {
    pub fn update(&mut self, new_val: i32) {
        let res = self.try_update(new_val);
        if res.is_err() {
            eprintln!("Update failed for input {}", new_val);
        }
    }

    fn try_update(&mut self, new_val: i32) -> Result<(), ()> {
        if new_val == 42 {
            Err(())
        } else {
            self.value = new_val;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        // Initialize the structure with a known value.
        let mut p = Processor { value: 10 };
        // Invoke the update with the triggering input that should cause a failure.
        p.update(42);
        // The contract: if the update fails (as it should when new_val == 42),
        // the stored value remains unchanged.
        assert_eq!(p.value, 10, "Final value must remain unchanged when update fails");
    }
}
