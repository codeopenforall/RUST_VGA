struct Calculator;

impl Calculator {
    pub fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        if denominator == 0 {
            Err("Division by zero encountered")
        } else {
            Ok(100 / denominator)
        }
    }
}

fn main() {
    let calc = Calculator;
    let thread_val = 0; // Example value, can be changed as needed

    match calc.execute(thread_val) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}