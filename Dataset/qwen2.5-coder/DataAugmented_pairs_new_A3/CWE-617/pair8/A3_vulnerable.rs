pub struct Manager {
    len: usize,
}

impl Manager {
    pub fn new(len: usize) -> Self {
        Manager { len }
    }

    pub fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        debug_assert!(idx < self.len, "Index out-of-bounds in debug mode");
        assert!(idx < self.len, "Index out-of-bounds by attacker input");
        Ok(())
    }
}

fn main() {
    let mut m = Manager::new(10);
    let _ = m.process(15);
    println!("Finished processing in vulnerable version");
}