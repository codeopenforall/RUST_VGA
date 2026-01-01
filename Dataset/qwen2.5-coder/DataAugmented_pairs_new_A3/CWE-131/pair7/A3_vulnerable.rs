use std::mem;

struct DataHandler {
    data: Vec<u8>,
}

impl DataHandler {
    pub fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: Vec::new() };
        handler.copy_content(input);
        handler
    }

    fn copy_content(&mut self, input: &str) {
        let copy_len = mem::size_of_val(&input); // Vulnerable line
        self.data.extend_from_slice(&input.as_bytes()[..copy_len]);
    }

    pub fn get_content(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
    }
}