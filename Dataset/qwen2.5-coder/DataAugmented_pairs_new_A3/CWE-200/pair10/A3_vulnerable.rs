use std::fmt;

#[derive(Debug)]
struct Credential {
    secret: String,
}

impl fmt::Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Credential {{ secret: {} }}", self.secret)
    }
}

struct Processor {
    credential: Credential,
}

impl Processor {
    fn new(credential: Credential) -> Self {
        Processor { credential }
    }

    fn process(&self) -> Result<(), String> {
        unsafe {
            let ptr = self.credential.secret.as_ptr();
            let len = self.credential.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8(slice).unwrap();
            Err(format!("Processing error with credential: {:?}", secret_str))?
        }
    }
}

fn main() {
    let cred = Credential { secret: "super_secret_1234".to_string() };
    let proc = Processor::new(cred);
    match proc.process() {
        Ok(_) => println!("Processing succeeded"),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}