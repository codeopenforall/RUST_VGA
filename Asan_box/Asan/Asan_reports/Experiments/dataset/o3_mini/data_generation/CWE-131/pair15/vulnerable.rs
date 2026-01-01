use std::mem;
use std::ptr;
use std::thread;
struct CustomBuffer {
    ptr: *mut u8,
    size: usize,
}
impl CustomBuffer {
    unsafe fn new(input: &str) -> CustomBuffer {
        let computed_size = mem::size_of_val(&input);
        let layout = std::alloc::Layout::from_size_align(computed_size, mem::align_of::<u8>())
            .expect("Invalid layout");
        let alloc = std::alloc::alloc(layout);
        if alloc.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr::copy_nonoverlapping(input.as_ptr(), alloc, input.len());
        CustomBuffer {
            ptr: alloc,
            size: computed_size,
        }
    }
    unsafe fn as_str(&self) -> &str {
        let slice = std::slice::from_raw_parts(self.ptr, self.size);
        std::str::from_utf8(slice).unwrap_or("")
    }
}
impl Drop for CustomBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = std::alloc::Layout::from_size_align(self.size, mem::align_of::<u8>()).unwrap();
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}
trait Processor {
    fn process(&self, text: &str) -> String;
}
struct ConcatProcessor;
impl Processor for ConcatProcessor {
    fn process(&self, text: &str) -> String {
        let half = text.len() / 2;
        let part1: String = text.chars().take(half).collect();
        let part2: String = text.chars().skip(half).collect();
        let handle1 = thread::spawn(move || part1);
        let handle2 = thread::spawn(move || part2);
        let res1 = handle1.join().unwrap_or_default();
        let res2 = handle2.join().unwrap_or_default();
        format!("{}{}", res1, res2)
    }
}
fn main() {
    let raw_buffer;
    unsafe {
        raw_buffer = CustomBuffer::new("This is a string with more than sixteen characters");
    }
    let processor = ConcatProcessor;
    let output = processor.process(unsafe { raw_buffer.as_str() });
    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_integrity() {
        // Input is longer than 16 bytes; the expected behavior is that the entire string is preserved.
        let input = "This is a string with more than sixteen characters";
        let buffer;
        unsafe {
            buffer = CustomBuffer::new(input);
        }
        let processor = ConcatProcessor;
        let result = processor.process(unsafe { buffer.as_str() });
        // In the vulnerable version, the output would be truncated (due to buffer size miscalculation),
        // so this assertion will fail. The fixed version produces the full string.
        assert_eq!(result, input, "The processed string does not match the original input");
    }
}
