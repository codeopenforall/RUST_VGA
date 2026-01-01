struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn compute_sum(&self, start: usize, count: usize) -> Result<i32, &'static str> {
        let mut sum = 0;
        unsafe {
            for i in 0..count {
                sum += *self.data.get_unchecked(start + i);
            }
        }
        Ok(sum)
    }
}

fn main() {
    let buffer = Buffer { data: vec![1, 2, 3, 4] };
    let result = buffer.compute_sum(0, 5);
    match result {
        Err(msg) => {
            println!("Error: {}", msg);
        }
        Ok(sum) => {
            println!("Sum: {}", sum);
        }
    }
}