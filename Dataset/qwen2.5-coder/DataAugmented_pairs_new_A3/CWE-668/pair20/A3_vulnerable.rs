struct Calculator;

impl Calculator {
    pub fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        unsafe {
            Ok(100 / denominator)
        }
    }
}

fn main() {
    let calc = Calculator;
    let thread_val = 0;
    let result = calc.execute(thread_val).unwrap();
    println!("Output: {}", result);
}