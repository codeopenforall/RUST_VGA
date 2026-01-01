use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::slice;
use std::thread;

fn process(input: &str) -> Vec<u8> {
    unsafe {
        let size = input.len();
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        std::ptr::copy_nonoverlapping(input.as_ptr(), ptr as *mut u8, size);
        let result = slice::from_raw_parts(ptr as *const u8, size).to_vec();
        dealloc(ptr, layout);
        result
    }
}

fn main() {
    let input = "This is a longer string that exceeds pointer size.";
    let handle = thread::spawn(move || {
        let res = process(input);
        println!("Output: {:?}", res);
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    // Import the appropriate module depending on which implementation is being linked.
    // The test must be run against the vulnerable binary (expected to fail)
    // and against the fixed binary (expected to pass).
    use super::*;

    #[test]
    fn test_process_function() {
        // Use an input where the correct length is more than the size of a reference (typically 8 bytes).
        let input = "0123456789"; // 10 bytes in ASCII
        let expected = input.as_bytes();

        let output = process(input);

        // The vulnerable version will only copy 8 bytes whereas the fixed version copies all 10 bytes.
        assert_eq!(output, expected, "The processed output should match the expected full input bytes.");
    }
}
