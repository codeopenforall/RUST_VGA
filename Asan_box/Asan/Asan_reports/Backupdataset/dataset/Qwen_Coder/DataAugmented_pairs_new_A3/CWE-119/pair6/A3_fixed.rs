struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}

impl MemoryHandler {
    pub fn new(capacity: usize) -> Self {
        let buffer = unsafe { std::alloc::alloc(std::alloc::Layout::array::<u32>(capacity).unwrap()) as *mut u32 };
        MemoryHandler { buffer, capacity }
    }

    pub fn write_checked(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.capacity {
            unsafe {
                *self.buffer.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }

    pub fn free(self) {
        unsafe {
            std::alloc::dealloc(self.buffer as *mut u8, std::alloc::Layout::array::<u32>(self.capacity).unwrap());
        }
    }
}

pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new(10);
    handler.write_checked(index, 99)?;
    let res = unsafe { handler.read(index) };
    handler.free();
    Ok(res)
}