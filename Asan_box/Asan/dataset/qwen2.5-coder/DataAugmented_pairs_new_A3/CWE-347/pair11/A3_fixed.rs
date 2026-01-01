struct Checker;

impl Checker {
    fn new() -> Self {
        Checker
    }

    fn authenticate(&self, key: &[u8; 32], data: &[u8], sign: &[u8; 64]) -> bool {
        let expected = Checker::produce_signature(key, data);
        expected == *sign
    }

    fn produce_signature(key: &[u8; 32], data: &[u8]) -> [u8; 64] {
        let mut signature = [0u8; 64];
        for i in 0..32 {
            signature[i] = key[i];
        }
        let filler = data.len() as u8;
        for i in 32..64 {
            signature[i] = filler;
        }
        signature
    }
}