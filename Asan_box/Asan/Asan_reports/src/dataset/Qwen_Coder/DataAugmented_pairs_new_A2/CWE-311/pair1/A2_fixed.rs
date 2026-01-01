use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    unsafe fn ptr_reveal(&self) -> *const u8 {
        self.secret.as_ptr()
    }
}

fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
}

fn process(conf: Arc<Mutex<Config>>) {
    let conf_clone = Arc::clone(&conf);
    thread::spawn(move || {
        let conf_lock = conf_clone.lock().unwrap();
        let secret_ptr = unsafe { conf_lock.ptr_reveal() };
        let secret_len = conf_lock.secret.len();
        let secret_slice = unsafe { std::slice::from_raw_parts(secret_ptr, secret_len) };
        for byte in secret_slice {
            print!("{:02x}", byte);
        }
        println!();
    })
    .join()
    .unwrap();
}

fn main() {
    let config = Config {
        secret: "my_super_secret".to_owned(),
    };
    let arc_conf = Arc::new(Mutex::new(config));
    let key: u8 = 0xAA;
    let plain_secret = {
        let conf_lock = arc_conf.lock().unwrap();
        let ptr = unsafe { conf_lock.ptr_reveal() };
        let len = conf_lock.secret.len();
        unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned()
        }
    };
    let encrypted_data = encrypt(&plain_secret, key);
    let mut file = File::create("secret.txt").expect("cannot create file");
    file.write_all(&encrypted_data)
        .expect("cannot write data");
    process(arc_conf);
    let decrypted = decrypt(&encrypted_data, key);
    assert_eq!(decrypted, plain_secret, "Decryption failed");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    // This test oracle checks that the sensitive secret is not stored in clear text.
    // For the vulnerable version, the file "secret.txt" will contain the plaintext
    // "my_super_secret", so the assertion will fail. In the fixed version, the file
    // contents are encrypted, so they will not match the plaintext.
    #[test]
    fn test_sensitive_storage_encryption() {
        // Remove any existing file.
        let _ = fs::remove_file("secret.txt");

        // Execute the program's main function.
        crate::main();

        // Read the contents of the file.
        let data = fs::read("secret.txt").expect("File 'secret.txt' should exist");
        let expected = b"my_super_secret";

        // The test asserts that the file content should NOT equal the sensitive plaintext.
        assert_ne!(data, expected, "Sensitive data is stored in clear text!");
    }
}
