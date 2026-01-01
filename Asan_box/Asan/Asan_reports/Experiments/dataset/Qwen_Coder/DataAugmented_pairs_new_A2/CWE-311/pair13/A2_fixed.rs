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

fn xor_cipher(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}

fn transmit(data: &[u8]) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(data)?;
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
    let encrypted = xor_cipher(&data, 0xAA);
    thread::spawn(move || {
        if let Err(e) = transmit(&encrypted) {
            eprintln!("Error during transmission: {:?}", e);
        }
    }).join().unwrap();
}

fn main() {
    let config = load_config("config.txt").expect("Could not load configuration");
    let shared = Arc::new(config);
    execute(shared);
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener};
    use std::sync::Arc;
    use std::io::{Read, Write};
    use std::thread;
    use std::time::Duration;
    use std::fs::File;
    use std::io::Write as IoWrite;

    // Helper function to simulate a server that captures the transmitted payload.
    fn run_server() -> Vec<u8> {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Server bind failed");
        // Accept one incoming connection.
        let (mut stream, _) = listener.accept().expect("Failed to accept connection");
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).expect("Failed to read from connection");
        buffer
    }

    #[test]
    fn test_transmission() {
        // Write a test configuration file with known sensitive data.
        let secret = "secret_data".to_string();
        let mut file = File::create("config.txt").expect("Failed to create test config file");
        file.write_all(secret.as_bytes()).expect("Failed to write test config");
        file.sync_all().expect("Sync failed");

        // Start a server thread to capture the outgoing data.
        let server_handle = thread::spawn(|| {
            run_server()
        });

        // Allow some time for the server to be ready.
        thread::sleep(Duration::from_millis(100));

        // Invoke the main function from the module under test.
        // For the vulnerable version, this will transmit the cleartext 'secret_data'.
        // For the fixed version, the transmitted data will be XOR-encrypted.
        super::main();

        let received = server_handle.join().expect("Server thread panicked");

        // The test asserts that the transmitted data should differ from the original cleartext.
        // If the cleartext is found, it indicates that the data was transmitted insecurely.
        assert_ne!(String::from_utf8_lossy(&received), secret, "Sensitive data was transmitted in cleartext!");
    }
}
