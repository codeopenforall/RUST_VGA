use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    value: u64,
}

impl Processor {
    fn new(input: &str) -> Result<Self, String> {
        let parsed = input.parse::<u64>().map_err(|e| format!("Parsing error: {}", e))?;
        Ok(Processor { value: parsed })
    }

    fn add_one(&mut self) {
        self.value += 1;
    }
}

fn process_input(input: &str) -> Result<u64, String> {
    let mut proc = Processor::new(input)?;
    proc.add_one();
    Ok(proc.value)
}

fn main() {
    let inputs = vec!["10", "20", "invalid"];
    let outputs = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::<String>::new()));

    let threads: Vec<_> = inputs.into_iter().map(|inp| {
        let out_clone = Arc::clone(&outputs);
        let err_clone = Arc::clone(&errors);
        let inp = inp.to_string();
        thread::spawn(move || {
            match process_input(&inp) {
                Ok(val) => {
                    let mut o = out_clone.lock().unwrap();
                    o.push(val);
                }
                Err(e) => {
                    let mut er = err_clone.lock().unwrap();
                    er.push(e);
                }
            }
        })
    }).collect();

    for th in threads {
        th.join().unwrap();
    }

    let collected_errors = errors.lock().unwrap();
    if !collected_errors.is_empty() {
        println!("Error occurred during processing: {:?}", *collected_errors);
        std::process::exit(1);
    } else {
        let out = outputs.lock().unwrap();
        let sum: u64 = out.iter().sum();
        println!("Sum: {}", sum);
    }
}

#[cfg(test)]
mod tests {
    use super::process_input;

    #[test]
    fn test_error_handling() {
        // The test oracle calls process_input with an invalid input.
        // For a correct implementation, it must return an error.
        let result = process_input("invalid");
        assert!(
            result.is_err(),
            "Expected process to error on invalid input but got Ok({:?})",
            result.ok()
        );
    }
}
