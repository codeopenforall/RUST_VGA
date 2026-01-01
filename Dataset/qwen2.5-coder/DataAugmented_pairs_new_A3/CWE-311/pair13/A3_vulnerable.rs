use std::io;
use std::net::TcpStream;

fn main() {
    let secret = "secret_data";
    transmit(secret).expect("Transmission failed");
}

fn transmit(data: &str) -> io::Result<()> {
    unsafe {
        let ptr = data.as_ptr();
        let len = data.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.write_all(slice)?;
    }
    Ok(())
}