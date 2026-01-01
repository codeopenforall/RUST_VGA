struct CryptoTool {
    expected_algo: String,
}

impl CryptoTool {
    pub fn new() -> Self {
        CryptoTool {
            expected_algo: "aes".to_string(),
        }
    }

    pub unsafe fn verify_signature(&self, msg: &[u8], sig: &[u8], algo: &str) -> bool {
        let ptr_algo = algo.as_ptr();
        let ptr_expected = self.expected_algo.as_ptr();
        if ptr_algo == ptr_expected {
            return sig.len() == msg.len();
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_algorithm() {
        let tool = CryptoTool::new();
        let msg = b"Hello, oracle!";
        let sig = vec![0u8; msg.len()];
        // Intentionally use an invalid algorithm.
        let result = unsafe { tool.verify_signature(msg, &sig, "rsa") };
        // The proper behavior is to reject an invalid algorithm.
        assert!(!result, "Verification should fail for an invalid algorithm");
    }
}
