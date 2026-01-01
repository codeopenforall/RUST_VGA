use std::ptr;

struct Verifier;

impl Verifier {
    pub fn check(msg: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 65 {
            return false;
        }
        unsafe {
            let sig_ptr = signature.as_ptr() as *const [u8; 65];
            let sig_arr: [u8; 65] = ptr::read(sig_ptr);
            let content_valid = sig_arr[1..].iter().any(|&b| b != 0);
            return content_valid;
        }
    }
}

fn main() {
    let message = b"Important confidential message";
    let public_key = [0u8; 32];
    let mut signature = [1u8; 65];
    signature[0] = 0xFF;
    let valid = Verifier::check(message, &public_key, &signature);
    if valid {
        println!("Signature verified (vulnerable).");
    } else {
        println!("Signature failed (vulnerable).");
    }
}

#[cfg(test)]
mod tests {
    use super::Verifier;

    #[test]
    fn test_signature_verification() {
        let message = b"Important confidential message";
        let public_key = [0u8; 32];
        // Create a forged signature: algorithm identifier is 0xFF which is not allowed, and the rest bytes are non-zero.
        let mut signature = [1u8; 65];
        signature[0] = 0xFF;
        // For the vulnerable version, this incorrect signature would incorrectly pass verification,
        // whereas the fixed version should reject it.
        let valid = Verifier::check(message, &public_key, &signature);
        assert!(!valid, "Signature verification MUST fail for invalid algorithm identifier");
    }
}
