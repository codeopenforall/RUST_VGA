use std::mem::size_of;

fn process() -> String {
    let original = String::from("This is a test message that exceeds the typical struct size.");
    let copy_len = size_of::<String>();
    let mut buffer = vec![0; copy_len];
    original.as_bytes().iter().take(copy_len).enumerate().for_each(|(i, &byte)| buffer[i] = byte);
    String::from_utf8(buffer).unwrap()
}

fn main() {
    println!("{}", process());
}