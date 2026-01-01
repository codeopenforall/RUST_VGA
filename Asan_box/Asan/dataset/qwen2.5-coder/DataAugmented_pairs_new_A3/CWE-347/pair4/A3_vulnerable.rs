use std::ptr;

struct Record {
    content: String,
    signature: [u8; 32],
}

impl Record {
    pub fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = unsafe { self.compute_expectation(key) };
        unsafe {
            let exp_ptr = expected.as_ptr();
            let sig_ptr = self.signature.as_ptr();
            for i in 0..32 {
                if *exp_ptr.add(i) != *sig_ptr.add(i) {
                    return false;
                }
            }
        }
        true
    }

    unsafe fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
        let bytes = self.content.as_bytes();
        let mut outcome = [0u8; 32];
        for i in 0..32 {
            let c = *bytes.get_unchecked(i % bytes.len());
            outcome[i] = c ^ key[i];
        }
        outcome
    }
}