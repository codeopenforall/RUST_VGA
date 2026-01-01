const KEY: u8 = 0xAB;

fn encrypt_data(secret: &String) -> String {
    secret
        .bytes()
        .map(|b| format!("{:02x}", b ^ KEY))
        .collect::<String>()
}

fn get_transmitted_data() -> String {
    let secret = String::from("myS3cr3t!");
    encrypt_data(&secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmission_must_not_be_plaintext() {
        let secret = "myS3cr3t!";
        let transmitted = get_transmitted_data();
        assert_ne!(transmitted, secret, "Failure: Transmitted data is in cleartext!");
    }
}