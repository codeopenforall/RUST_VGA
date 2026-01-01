use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let plain_secret = "my_super_secret";
    let mut file = File::create("secret.txt")?;

    // Intentionally write the plain secret to the file
    file.write_all(plain_secret.as_bytes())?;

    Ok(())
}