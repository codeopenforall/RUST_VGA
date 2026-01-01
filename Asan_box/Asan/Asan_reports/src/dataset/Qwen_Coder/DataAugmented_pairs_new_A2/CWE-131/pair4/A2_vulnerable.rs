use std::mem;
use std::ptr;
use std::thread;

pub fn process() -> Vec<u8> {
    let input = String::from("1234567890123456");
    let mut buf = [0u8; 16];
    unsafe {
        let size = mem::size_of_val(&buf);
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), size);
        // Introducing a defect: writing beyond the allocated buffer
        *buf.as_mut_ptr().add(size - 1) = 0;
    }
    buf.to_vec()
}

fn run_app() {
    let data = process();
    println!("Buffer content: {:?}", data);
}

fn main() {
    let handle = thread::spawn(|| {
        run_app();
    });
    handle.join().unwrap();
}

//////////////////// Test Oracle ////////////////////
// This test oracle function uses a fixed triggering input to validate the proper buffer size and null termination.
// For the corrected code, the returned byte vector must have a length of 17 with the last byte equal to 0.
// For the vulnerable code, the test is expected to fail (e.g. incorrect vector length or undefined behavior).
// The test uses explicit assertions to check the invariants.

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_oracle() {
        let data = process();
        // The expected buffer length is 17 bytes (16 data bytes + 1 null terminator).
        assert_eq!(data.len(), 17, "Buffer length should be 17 bytes");
        // Validate that the null terminator is present at the correct position.
        assert_eq!(data[16], 0, "Buffer must be null terminated");
    }
}
