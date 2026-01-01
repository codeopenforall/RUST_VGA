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