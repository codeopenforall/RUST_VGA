use std::fs::File;
use std::io::Write;

struct Data {
    secret: String,
}

fn encrypt(data: &[u8]) -> Vec<u8> {
    let key: u8 = 0xAA; 
    data.iter().map(|b| b ^ key).collect()
}

fn main() {
    let data = Data {
        secret: String::from("SensitiveSecret"),
    };
    let raw = data.secret.as_bytes();
    let cipher = encrypt(raw);
    let mut file = File::create("output.txt")
        .expect("Unable to create output file");
    file.write_all(&cipher)
        .expect("Failed to write encrypted data to file");
}