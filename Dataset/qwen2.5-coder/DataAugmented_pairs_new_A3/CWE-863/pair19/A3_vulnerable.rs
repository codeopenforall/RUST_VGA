pub struct ResourceManager;

impl ResourceManager {
    pub fn execute(&self, i: usize) -> Result<u32, &'static str> {
        let mut buffer = vec![0u32; 50];
        unsafe {
            let ptr = buffer.as_mut_ptr();
            *ptr.add(i) = 42;
        }
        Ok(buffer[i])
    }
}