pub struct Formatter {
    pub secret: String,
}

impl Formatter {
    pub fn run(&self, user_input: &str) -> String {
        unsafe {
            let fmt = user_input;
            let mut result = String::new();
            let mut remainder = fmt;
            while let Some(pos) = remainder.find("{}") {
                result.push_str(&remainder[..pos]);
                result.push_str(&self.secret);
                remainder = &remainder[pos + 2..];
            }
            result.push_str(remainder);
            result
        }
    }
}