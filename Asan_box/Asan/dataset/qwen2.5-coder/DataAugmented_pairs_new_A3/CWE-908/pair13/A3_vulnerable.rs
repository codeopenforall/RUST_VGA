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