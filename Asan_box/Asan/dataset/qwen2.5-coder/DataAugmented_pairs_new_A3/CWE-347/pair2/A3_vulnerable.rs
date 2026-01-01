struct DataPacket {
    message: Vec<u8>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}

impl DataPacket {
    pub fn new(message: &[u8], signature: &[u8], public_key: &[u8]) -> Self {
        DataPacket {
            message: message.to_vec(),
            signature: signature.to_vec(),
            public_key: public_key.to_vec(),
        }
    }

    pub fn verify(&self) -> bool {
        unsafe {
            if self.signature.len() == 64 {
                let ptr = self.signature.as_ptr() as *const u64;
                let _value = *ptr;
                return true;
            }
        }
        false
    }
}

fn main() {}