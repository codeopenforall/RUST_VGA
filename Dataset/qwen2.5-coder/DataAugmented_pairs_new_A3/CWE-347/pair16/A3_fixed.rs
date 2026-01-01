struct DataPacket {
    msg: String,
    sig: String,
    method: String,
}

impl DataPacket {
    fn new(msg: &str, sig: &str, method: &str) -> Self {
        DataPacket {
            msg: msg.to_string(),
            sig: sig.to_string(),
            method: method.to_string(),
        }
    }

    fn validate(&self) -> bool {
        match self.method.as_str() {
            "reverse" => {
                let expected: String = self.msg.chars().rev().collect();
                if expected == self.sig {
                    println!("Validation succeeded.");
                    true
                } else {
                    println!("Validation failed.");
                    false
                }
            }
            _ => {
                println!("Unsupported validation method.");
                false
            }
        }
    }
}