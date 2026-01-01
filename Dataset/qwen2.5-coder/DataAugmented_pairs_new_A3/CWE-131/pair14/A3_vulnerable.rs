pub struct Handler;

impl Handler {
    pub fn process(input: &str) -> Vec<u8> {
        // Intentionally using size_of_val which ignores the actual length of the string
        let size = std::mem::size_of_val(&input);
        input[..size].as_bytes().to_vec()
    }
}