use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::net::TcpStream;
use std::sync::Arc;
struct Configuration {
    data: String,
}
impl Configuration {
    fn new(data: String) -> Self {
        Configuration { data }
    }
}
fn transmit(data: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let encrypted_data: Vec<u8> = data.bytes().map(|b| b ^ 0xFF).collect(); // XOR encryption
    stream.write_all(&encrypted_data)?;
    Ok(())
}
fn load_config(path: &str) -> io::Result<Configuration> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(Configuration::new(buffer))
}
fn execute(config: Arc<Configuration>) {
    let data = config.data.clone();
    thread::spawn(move || {
         if let Err(e) = transmit(&data) {
              eprintln!("Error during transmission: {:?}", e);
         }
    }).join().unwrap();
}
fn main() {
    let config = load_config("config.txt").expect("Could not load configuration");
    let shared = Arc::new(config);
    execute(shared);
}