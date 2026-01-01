use std::thread;

struct DataHandler;

impl DataHandler {
    pub fn combine(&self, input: &[u8]) -> Vec<u8> {
        let mut output = input.to_vec();
        output.push(0);
        output
    }
}

fn main() {
    let handler = DataHandler;
    let input = b"test";
    let handle = thread::spawn(move || {
        let result = handler.combine(input);
        println!("Combined output: {:?}", result);
    });
    handle.join().unwrap();
}