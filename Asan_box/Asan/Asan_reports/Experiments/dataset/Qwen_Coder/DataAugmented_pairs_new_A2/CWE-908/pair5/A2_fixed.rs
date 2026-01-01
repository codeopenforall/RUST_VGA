use std::thread;

struct Resources {
    title: String,
    amount: u32,
}

impl Resources {
    fn new_complete() -> Self {
        Resources {
            title: String::from("configuration"),
            amount: 0,
        }
    }
}

fn run_app() -> u32 {
    let config = Resources::new_complete();
    let handle = thread::spawn(move || config.amount);
    handle.join().unwrap()
}

fn main() {
    let result = run_app();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // Bring in the run_app function from the module under test.
    // When testing, the vulnerable version would likely yield an unpredictable result
    // (or even panic), whereas the corrected implementation must return exactly 0.
    use super::run_app;

    #[test]
    fn test_run_app_result() {
        let res = run_app();
        // The test expects the "amount" field to be properly initialized to 0.
        assert_eq!(res, 0, "Test failed: Expected result to be 0 for properly initialized resource.");
    }
}
