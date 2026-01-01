use std::ptr;

struct Processor {
    value: i32,
}

impl Processor {
    fn update(&mut self, new_val: i32) {
        let ptr: *mut i32 = &mut self.value;
        let res = unsafe { modify(ptr, new_val) };
        if res.is_err() {
            self.value = new_val;
        }
    }
}

unsafe fn modify(ptr: *mut i32, new_val: i32) -> Result<(), i32> {
    if new_val == 42 {
        Err(new_val)
    } else {
        *ptr = new_val;
        Ok(())
    }
}

fn main() {
    let mut p = Processor { value: 10 };
    p.update(42);
    println!("Value: {}", p.value);
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
