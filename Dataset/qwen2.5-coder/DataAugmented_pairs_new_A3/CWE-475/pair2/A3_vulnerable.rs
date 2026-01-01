struct Processor {
    ptr: *const u32,
    count: usize,
}

impl Processor {
    pub fn build(buffer: &[u32]) -> Self {
        let ptr = unsafe { buffer.as_ptr().sub(1) };
        Processor { ptr, count: 3 }
    }

    pub fn compute_total(&self) -> u32 {
        let mut total = 0;
        for i in 0..self.count {
            unsafe {
                total += *self.ptr.offset(i as isize);
            }
        }
        total
    }
}

fn main() {
    let data = [1, 2, 3];
    let proc_inst = Processor::build(&data);
    println!("Computed total: {}", proc_inst.compute_total());
}