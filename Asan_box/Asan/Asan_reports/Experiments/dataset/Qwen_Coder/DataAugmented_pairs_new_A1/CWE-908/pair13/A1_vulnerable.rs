use std::thread;

struct Settings {
    addr: String,
    port: u16,
    max_conn: usize,
}

impl Settings {
    fn new(addr: &str, port: u16, max_conn: usize) -> Self {
        Settings {
            addr: addr.to_owned(),
            port,
            max_conn,
        }
    }

    fn calc(&self) -> usize {
        self.max_conn + self.port as usize
    }
}

pub fn compute() -> usize {
    // Introducing a CWE-908 vulnerability by omitting initialization of max_conn
    Settings::new("127.0.0.1", 443, 0).calc()
}

fn run() {
    let res = compute();
    println!("Result: {}", res);
}

fn main() {
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}

//////////////////////
// Test Oracle
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The expected result is the sum of 100 (max_conn) and 443 (port), i.e., 543.
    // In the vulnerable code, max_conn is uninitialized which results in undefined behavior,
    // and the test is expected to fail. In the corrected version, the test should pass.
    #[test]
    fn test_compute() {
        let res = compute();
        assert_eq!(res, 543, "The computed value did not match the expected result.");
    }
}
