#![allow(dead_code)]

pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
    unsafe {
        let expected_len = 32;
        let mut buffer = [0u8; 32];
        let copy_len = if sig.len() < expected_len { sig.len() } else { expected_len };
        std::ptr::copy_nonoverlapping(sig.as_ptr(), buffer.as_mut_ptr(), copy_len);
        true
    }
}

fn concurrent_check(msg: std::sync::Arc<Vec<u8>>, sig: std::sync::Arc<Vec<u8>>, pubkey: std::sync::Arc<Vec<u8>>) -> bool {
    check(&msg, &sig, &pubkey)
}

//////////////////// oracle_test.rs ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Test oracle that ensures a forged signature is rejected.
    // In the vulnerable version, the forged signature would be accepted,
    // causing this test to fail. In the fixed version, the forged signature is
    // correctly rejected, and the test passes.
    #[test]
    fn test_signature_verification() {
        // Prepare test data.
        let message = b"Test message".to_vec();
        let pubkey = b"TestPublicKey".to_vec();
        // Forge a signature that is 32 bytes of zeros.
        let forged_signature = vec![0u8; 32];
        let result = concurrent_check(Arc::new(message), Arc::new(forged_signature), Arc::new(pubkey));
        assert!(!result, "Forged signature should not verify");
    }
}
