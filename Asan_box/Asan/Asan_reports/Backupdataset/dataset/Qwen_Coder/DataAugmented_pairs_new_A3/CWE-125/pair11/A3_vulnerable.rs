use std::sync::Arc;

struct Processor {
    data: Arc<Vec<i32>>,
}

impl Processor {
    fn compute(&self, start: usize, count: usize) -> i32 {
        let mut total = 0;
        unsafe {
            for i in 0..count {
                total += *self.data.get_unchecked(start + i);
            }
        }
        total
    }
}

fn main() {
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    let result = proc_inst.compute(3, 2);
    println!("Computed sum: {}", result);
}