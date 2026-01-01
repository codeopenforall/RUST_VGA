pub struct Formatter {
    pub secret: String,
}

impl Formatter {
    pub fn run(&self, user_input: &str) -> String {
        let escaped = user_input.replace("{", "{{").replace("}", "}}");
        format!("{}{}", escaped, self.secret)
    }
}