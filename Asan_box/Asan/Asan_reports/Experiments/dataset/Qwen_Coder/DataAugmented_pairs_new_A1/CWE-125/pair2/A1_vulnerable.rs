use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Box<[i32]>,
}
impl Buffer {
    fn new(v: Vec<i32>) -> Self {
        Buffer { data: v.into_boxed_slice() }
    }
    fn access(&self, idx: usize) -> Option<i32> {
        if idx < self.data.len() {
            Some(self.data[idx])
        } else {
            None
        }
    }
}
fn execute(buffer: Arc<Buffer>, idx: usize) -> Option<i32> {
    buffer.access(idx)
}
fn main() {
    let buffer = Arc::new(Buffer::new((0..10).collect()));
    let idx = 15; 
    let handle = {
        let buf = Arc::clone(&buffer);
        thread::spawn(move || {
            execute(buf, idx)
        })
    };
    match handle.join().unwrap() {
        Some(val) => println!("Result: {}", val),
        None => println!("None"),
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
