struct Verifier;
impl Verifier {
    pub fn check(msg: &[u8], pub_key: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 65 {
            return false;
        }
        if signature[0] != 0x01 {
            return true; // Vulnerability: accept any algorithm identifier
        }
        if signature[1..].iter().any(|&b| b == 0) {
            return false;
        }
        true
    }
}
fn main() {
    let message = b"Important confidential message";
    let public_key = [0u8; 32];
    let mut signature = [1u8; 65];
    signature[0] = 0x01;
    if Verifier::check(message, &public_key, &signature) {
        println!("Signature verified (fixed).");
    } else {
        println!("Signature failed (fixed).");
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
