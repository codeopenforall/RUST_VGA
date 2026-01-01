use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    let buf = 100;
    file.write_all(&[buf as u8])?;
    println!("Operation completed in insecure mode.");
    Ok(())
}