struct Calculator {
    numerator: i32,
    denominator: i32,
}

impl Calculator {
    fn execute(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Error: Division by zero encountered");
        }
        Ok(self.numerator / self.denominator)
    }
}

fn main() {
    let calculator = Calculator {
        numerator: 10,
        denominator: 0,
    };

    let handler = std::thread::spawn(move || {
        calculator.execute()
    });

    let result = handler.join().expect("Thread panicked unexpectedly");

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("{}", err),
    }
}