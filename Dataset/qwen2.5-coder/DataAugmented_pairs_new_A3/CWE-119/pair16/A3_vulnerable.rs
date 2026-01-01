use std::ptr;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub unsafe fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        let ptr = self.data.as_mut_ptr();
        *ptr.add(idx) = val; 
        Ok(())
    }
}

fn run() -> Result<(), &'static str> {
    let mut buf = Buffer { data: vec![0; 10] };
    unsafe { buf.update(10, 100) }?;
    Ok(())
}

fn main() {
    // This main function is just a placeholder and not part of the test.
    // The actual testing is done in demo_test.rs.
}