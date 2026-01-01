fn transmit_data(secret: &String) -> String {
    let mut result = String::new();
    unsafe {
        let raw = secret.as_ptr();
        let slice = std::slice::from_raw_parts(raw, secret.len());
        result = String::from_utf8_lossy(slice).to_string();
    }
    result
}

fn get_transmitted_data() -> String {
    let secret = String::from("myS3cr3t!");
    transmit_data(&secret)
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

/*
The following test oracle function verifies that the transmitted data does not equal
the cleartext secret. When executed against the insecure implementation, the test will fail
(as the plain secret "myS3cr3t!" is transmitted), while it will pass against the secure version.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmission_must_not_be_plaintext() {
        // The expected sensitive secret.
        let secret = "myS3cr3t!";
        // Invoke the transmission function from the implementation.
        let transmitted = get_transmitted_data();
        // The transmitted data must not match the cleartext secret.
        assert_ne!(
            transmitted, secret,
            "Failure: Transmitted data is in cleartext!"
        );
    }
}
