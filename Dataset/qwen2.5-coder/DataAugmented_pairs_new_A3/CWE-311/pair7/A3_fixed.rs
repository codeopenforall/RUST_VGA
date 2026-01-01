use std::fs::File;
use std::io::Write;

struct Data {
    secret: String,
}

fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn main() {
    let data = Data {
        secret: String::from("SuperSecretPassword123!"),
    };

    let key: u8 = 0xAA;
    let encrypted = xor_encrypt(data.secret.as_bytes(), key);

    let mut file = File::create("secret.txt").expect("Failed to create file");
    file.write_all(&encrypted).expect("Failed to write file");
}