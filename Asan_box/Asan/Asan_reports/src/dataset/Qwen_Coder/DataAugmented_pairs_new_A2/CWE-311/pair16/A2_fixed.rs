use std::env;
use std::thread;

struct Service {
    data: String,
}

impl Service {
    fn new(data: String) -> Self {
        Service { data }
    }

    unsafe fn raw_copy(ptr: *const u8, len: usize) -> Vec<u8> {
        let slice = std::slice::from_raw_parts(ptr, len);
        slice.to_vec()
    }

    fn xor_enc(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }

    fn process(&self) -> String {
        let data_bytes = self.data.as_bytes();
        let len = data_bytes.len();
        let copied = unsafe { Service::raw_copy(data_bytes.as_ptr(), len) };
        let encrypted = Service::xor_enc(&copied, 0xAA);
        encrypted.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

fn main() {
    let secret = env::var("SECRET").unwrap_or_else(|_| "sensitive_data".to_string());
    let service = Service::new(secret);
    let handle = thread::spawn(move || {
        service.process()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle instantiates the service with a known secret,
    // calls the process method and asserts that the output does NOT match
    // the original input. This test should fail for the vulnerable version
    // (which returns the clear text secret) and pass for the fixed version.
    #[test]
    fn test_secret_processing() {
        let input = "test_secret";
        let service = Service::new(input.to_string());
        let processed = service.process();
        // The processed result should differ from the original secret.
        // In the vulnerable version, they would be equal.
        assert_ne!(processed, input, "Sensitive data is not encrypted!");
    }
}
