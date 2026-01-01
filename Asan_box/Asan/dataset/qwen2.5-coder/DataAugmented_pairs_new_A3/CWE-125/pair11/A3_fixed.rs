struct Processor {
    data: std::sync::Arc<Vec<i32>>,
}

impl Processor {
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str> {
        let slice = &self.data;
        if start.checked_add(count).map_or(true, |end| end > slice.len()) {
            return Err("Index out-of-bounds");
        }
        let mut total = 0;
        for i in 0..count {
            total += slice.get(start + i).unwrap();
        }
        Ok(total)
    }
}

fn main() {
    let proc_inst = Processor { data: std::sync::Arc::new(vec![10, 20, 30, 40]) };
    match proc_inst.compute(3, 2) {
        Ok(result) => println!("Computed sum: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}