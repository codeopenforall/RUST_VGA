#![allow(dead_code)]
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Packet {
    tag: u32,
    content: String,
}

impl Packet {
    pub unsafe fn pack(&self) -> Vec<u8> {
        let header_size = mem::size_of_val(&self.tag);
        let content_size = self.content.len(); // Incorrectly using len() instead of size_of_val()
        let total_size = header_size + content_size;
        let mut buffer = Vec::with_capacity(total_size);
        buffer.set_len(total_size);
        ptr::copy_nonoverlapping(
            &self.tag as *const u32 as *const u8,
            buffer.as_mut_ptr(),
            header_size,
        );
        ptr::copy_nonoverlapping(
            self.content.as_ptr(),
            buffer.as_mut_ptr().add(header_size),
            content_size,
        );
        buffer
    }
}

fn main() {
    let pkt = Arc::new(Packet {
        tag: 0xDEADBEEF,
        content: String::from("Hello"),
    });
    let mut handles = vec![];
    for _ in 0..4 {
        let pkt_clone = pkt.clone();
        handles.push(thread::spawn(move || {
            unsafe {
                let buf = pkt_clone.pack();
                println!("Packed bytes: {:?}", buf);
            }
        }));
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////

// This test oracle is designed to fail for the vulnerable implementation and pass for the corrected one.
// It creates a Packet with "Hello" as the content. The expected serialized length is 4 bytes for the u32 header plus 5 bytes for "Hello", totaling 9 bytes.

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // Stub definition for Packet and pack function will be linked against either vulnerable or corrected implementations.
    // Ensure that the implementation under test exposes the same interface.

    #[test]
    fn test_packet_pack_length() {
        let pkt = Packet {
            tag: 0xDEADBEEF,
            content: String::from("Hello"),
        };

        unsafe {
            let buf = pkt.pack();
            // The header is a u32 (4 bytes) and the content "Hello" is 5 bytes.
            let expected_length = mem::size_of_val(&pkt.tag) + pkt.content.len();
            // The vulnerable code incorrectly calculates the content size, leading to an unexpected total length.
            assert_eq!(buf.len(), expected_length, "Packed buffer length mismatch!");
        }
    }
}
