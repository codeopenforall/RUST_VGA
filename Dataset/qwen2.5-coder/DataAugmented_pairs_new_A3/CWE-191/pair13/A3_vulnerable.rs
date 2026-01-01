pub struct Counter {
    value: u32,
}

impl Counter {
    pub unsafe fn reduce(&mut self, amount: u32) {
        self.value = self.value - amount;
    }
}

pub fn execute(amount: u32) -> u32 {
    let mut counter = Counter { value: 10 };
    unsafe {
        counter.reduce(amount);
    }
    counter.value
}