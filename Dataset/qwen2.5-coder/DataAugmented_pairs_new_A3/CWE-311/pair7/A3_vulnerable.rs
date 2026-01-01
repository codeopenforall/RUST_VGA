use std::fs::File;
use std::io::Write;

struct SecretData {
    secret: String,
}

impl SecretData {
    fn new(secret: &str) -> Self {
        SecretData {
            secret: secret.to_string(),
        }
    }
}

fn main() {
    let data = SecretData::new("SuperSecretPassword123!");
    let mut file = File::create("secret.txt").expect("Failed to create file");

    let slice = data.secret.as_bytes();
    let _leak = String::from_utf8_lossy(slice);

    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
}