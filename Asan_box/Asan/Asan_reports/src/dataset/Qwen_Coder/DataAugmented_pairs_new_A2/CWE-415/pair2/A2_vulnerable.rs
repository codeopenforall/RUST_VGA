use std::sync::atomic::{AtomicUsize, Ordering};

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Data {
    value: u32,
}

impl Drop for Data {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

fn handle_data() {
    let data = Box::new(Data { value: 42 });
    let raw_ptr = Box::into_raw(data);
    unsafe {
        drop(Box::from_raw(raw_ptr));
        // Introducing a defect: attempting to drop the same pointer again
        drop(Box::from_raw(raw_ptr));
    }
}

pub fn run_simulation() -> usize {
    DROP_COUNT.store(0, Ordering::SeqCst);
    handle_data();
    DROP_COUNT.load(Ordering::SeqCst)
}

fn main() {
    let drops = run_simulation();
    println!("Drop count: {}", drops);
}

#[cfg(test)]
mod tests {
    use super::run_simulation;

    #[test]
    fn test_deallocation() {
        // When run_simulation is executed, it should deallocate the memory exactly once.
        let drop_result = run_simulation();
        assert_eq!(
            drop_result, 
            1, 
            "Expected exactly one drop (proper deallocation). A different count indicates a double free vulnerability."
        );
    }
}
