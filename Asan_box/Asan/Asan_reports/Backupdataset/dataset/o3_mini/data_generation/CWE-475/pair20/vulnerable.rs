use std::thread;
struct Container {
    payload: Vec<u8>,
}
impl Container {
    fn compute(&self) -> Option<u32> {
        let mut sum = 0u32;
        unsafe {
            let ptr = self.payload.as_ptr() as *const u32;
            for i in 0..3 {
                sum = sum.wrapping_add(ptr.add(i).read());
            }
        }
        Some(sum)
    }
}
fn run() {
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let container = Container { payload: data };
    let handle = thread::spawn(move || {
        container.compute()
    });
    let result = handle.join().unwrap();
    println!("Computed sum: {:?}", result);
}
fn main() {
    run();
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
