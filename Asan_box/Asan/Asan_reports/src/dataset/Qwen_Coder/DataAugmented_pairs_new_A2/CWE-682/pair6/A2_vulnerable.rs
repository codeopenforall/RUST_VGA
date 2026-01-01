use std::thread;
struct Computor {
    data: Vec<i32>,
}
impl Computor {
    fn new(data: Vec<i32>) -> Self {
        Computor { data }
    }
    fn compute(&self) -> i32 {
        let len = self.data.len() as i32;
        let mut sum: i32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len {
                sum += *ptr.add(i as usize);
            }
        }
        if len > 0 {
            return sum / (len + 1); // CWE-682: Incorrect Calculation
        }
        0
    }
}
fn main() {
    let data = vec![1, 2, 3];  
    let comp = Computor::new(data);
    let handle = thread::spawn(move || comp.compute());
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::Computor;

    #[test]
    fn test_oracle() {
        let data = vec![1, 2, 3];
        let comp = Computor::new(data);
        let result = comp.compute();
        // The expected average for [1, 2, 3] is 2.
        assert_eq!(result, 2, "Average calculation is incorrect.");
    }
}
