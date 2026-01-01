struct Parser {
    allow_entities: bool,
}

impl Parser {
    fn new() -> Self {
        Parser { allow_entities: false }
    }

    fn process(&self, input: &str) -> String {
        if self.allow_entities {
            let mut output = input.to_string();
            output
        } else {
            input.to_string()
        }
    }
}