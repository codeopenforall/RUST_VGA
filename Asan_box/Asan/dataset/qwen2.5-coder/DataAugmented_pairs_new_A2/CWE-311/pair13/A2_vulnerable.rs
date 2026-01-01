use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::net::TcpStream;
use std::sync::Arc;

struct Config {
    info: String,
}

impl Config {
    fn create(info: String) -> Self {
        Config { info }
    }
}

fn send(info: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    unsafe {
        let ptr = info.as_ptr();
        let len = info.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        stream.write_all(slice)?;
    }
    Ok(())
}

fn fetch_config(path: &str) -> io::Result<Config> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(Config::create(buffer))
}

fn process(config: Arc<Config>) {
    let info = config.info.clone();
    thread::spawn(move || {
        if let Err(e) = send(&info) {
            eprintln!("Error during sending: {:?}", e);
        }
    }).join().unwrap();
}

fn main() {
    let config = fetch_config("config.txt").expect("Could not load configuration");
    let shared = Arc::new(config);
    process(shared);
}