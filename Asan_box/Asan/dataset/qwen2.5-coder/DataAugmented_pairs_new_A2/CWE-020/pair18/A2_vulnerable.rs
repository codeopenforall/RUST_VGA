#![allow(dead_code)]
use std::env;

struct Container {
    data: Vec<u8>,
}

impl Container {
    fn new(input: &[u8]) -> Option<Container> {
        Some(Container { data: input.to_vec() })
    }

    fn extract(&self, start: usize, length: usize) -> String {
        unsafe {
            let ptr = self.data.as_ptr().offset(start as isize);
            let slice = std::slice::from_raw_parts(ptr, length);
            std::str::from_utf8_unchecked(slice).to_string()
        }
    }
}

fn main() {
    let input_data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let container = Container::new(input_data).expect("Failed to create container");
    let user_start = 20;
    let user_length = 10;
    let extracted = container.extract(user_start, user_length);
    println!("Extracted: {}", extracted);
}