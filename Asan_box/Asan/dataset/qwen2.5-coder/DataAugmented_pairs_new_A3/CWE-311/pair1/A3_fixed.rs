use std::fs::File;
use std::io::{self, Write};

fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
}

fn main() -> io::Result<()> {
    let plain_secret = "my_super_secret";
    let key: u8 = 0xAA;
    let encrypted_data = encrypt(plain_secret, key);

    let mut file = File::create("secret.txt")?;
    file.write_all(&encrypted_data)?;

    Ok(())
}