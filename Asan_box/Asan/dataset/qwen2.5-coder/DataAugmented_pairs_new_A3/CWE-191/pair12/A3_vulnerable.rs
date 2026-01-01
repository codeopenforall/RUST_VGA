pub struct Data {
    count: u32,
}

impl Data {
    pub unsafe fn reduce(&mut self, sub: u32) -> u32 {
        self.count = self.count - sub;  
        self.count
    }
}

pub fn execute_subtraction() -> Result<u32, &'static str> {
    let mut data = Data { count: 5 };
    let res = unsafe { data.reduce(10) }; 
    Ok(res)
}