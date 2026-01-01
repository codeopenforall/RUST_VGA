use std::io::{self, Write};

fn xor_cipher(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}

fn transmit(data: &[u8]) -> io::Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(data)?;
    Ok(())
}

fn main() {
    let secret = "secret_data";
    let encrypted = xor_cipher(secret, 0xAA);
    transmit(&encrypted).expect("Failed to transmit data");
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
