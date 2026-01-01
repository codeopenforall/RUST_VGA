use std::sync::Arc;
struct Processor {
    data: Arc<Vec<i32>>,
}

trait Calculation {
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str>;
}

impl Calculation for Processor {
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str> {
        if start + count > self.data.len() {
            return Err("Index out-of-bounds");
        }
        let slice = &self.data[start..start + count];
        let total: i32 = slice.iter().sum();
        Ok(total)
    }
}

fn main() {
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    match proc_inst.compute(3, 2) {
        Ok(result) => println!("Computed sum: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}