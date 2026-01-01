use std::thread;

struct Divider {
    numerator: i32,
    denominator: i32,
}

impl Divider {
    fn new(numerator: i32, denominator: i32) -> Self {
        Divider { numerator, denominator }
    }

    fn execute(&self) -> i32 {
        self.numerator / self.denominator
    }
}

fn main() {
    let divider = Divider::new(10, 0);
    let handler = thread::spawn(move || {
        divider.execute()
    });

    let result = handler.join().expect("Thread panicked due to divide-by-zero");
    println!("Result: {}", result);
}