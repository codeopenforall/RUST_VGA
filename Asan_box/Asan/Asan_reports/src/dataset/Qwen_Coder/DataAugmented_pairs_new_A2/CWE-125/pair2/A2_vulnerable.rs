use std::sync::Arc;
use std::thread;

struct DataStore {
    elements: Box<[i32]>,
}

impl DataStore {
    fn initialize(data: Vec<i32>) -> Self {
        DataStore { elements: data.into_boxed_slice() }
    }
    fn retrieve(&self, index: usize) -> Option<i32> {
        unsafe {
            Some(*self.elements.get_unchecked(index))
        }
    }
}

fn perform_retrieval(store: Arc<DataStore>, index: usize) -> Option<i32> {
    store.retrieve(index)
}

fn main() {
    let store = Arc::new(DataStore::initialize((0..10).collect()));
    let index = 15;
    let task_handle = {
        let ds = Arc::clone(&store);
        thread::spawn(move || {
            perform_retrieval(ds, index)
        })
    };
    match task_handle.join().unwrap() {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found"),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    // This test verifies that accessing an out-of-bound element returns None.
    // In the unsafe implementation the behavior is undefined (typically yielding Some(value) or panicking),
    // so this test will fail for that version.
    #[test]
    fn test_out_of_bound_access() {
        let buffer = Arc::new(crate::Buffer::new((0..10).collect()));
        let handle = {
            let buf = Arc::clone(&buffer);
            thread::spawn(move || {
                buf.access(15)
            })
        };
        let result = handle.join().unwrap();
        assert_eq!(result, None, "Expected None for an out-of-bound access");
    }
}
