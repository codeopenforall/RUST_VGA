struct CryptoTool {
    expected_algo: &'static str,
}

impl CryptoTool {
    fn new() -> Self {
        CryptoTool {
            expected_algo: "aes",
        }
    }

    unsafe fn verify_signature(&self, msg: &[u8], sig: &[u8], algo: &str) -> bool {
        if algo != self.expected_algo {
            return false;
        }
        sig.len() == msg.len()
    }
}