use std::sync::{Arc, Mutex};

struct Processor {
    data: Vec<u8>,
}

impl Processor {
    pub unsafe fn append(&mut self, item: u8) {
        unsafe {
            self.data.push(item);
        }
    }
}

fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    let mut proc = proc.lock().unwrap();
    for byte in input {
        unsafe {
            proc.append(*byte);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::{simulate_input, Processor};

    #[test]
    fn test_resource_enforcement() {
        let proc = Arc::new(Mutex::new(Processor { data: Vec::new() }));
        let excessive_input = vec![42u8; 1500];
        
        let outcome = simulate_input(&excessive_input, &proc);
        
        assert!(outcome.is_err(), "Resource limit not enforced as expected");
    }
}