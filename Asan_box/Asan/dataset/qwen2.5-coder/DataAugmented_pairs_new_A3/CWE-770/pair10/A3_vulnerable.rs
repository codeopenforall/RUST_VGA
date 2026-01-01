struct Manager;

impl Manager {
    unsafe fn unsafe_allocate(&self, size: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size);
        vec.set_len(size);
        for elem in vec.iter_mut() {
            *elem = 1;
        }
        vec
    }

    fn process(&self, count: usize) -> Result<(), String> {
        let data = unsafe { self.unsafe_allocate(count) };
        let mut handles = Vec::new();
        let mut total: u64 = 0;

        for _ in 0..count {
            handles.push(data.clone());
            total += data.len() as u64;
        }

        Ok(())
    }
}

struct Processor;

impl Processor {
    fn process(&self, count: usize) -> Result<(), String> {
        let manager = Manager;
        manager.process(count)
    }
}