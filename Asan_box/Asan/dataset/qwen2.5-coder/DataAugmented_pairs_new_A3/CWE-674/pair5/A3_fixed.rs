use std::ptr;
struct Parser {
    data: *const u8,
    len: usize,
    pos: usize,
    max_depth: usize,
    current_depth: usize,
}
impl Parser {
    fn new(input: &str) -> Self {
        let bytes = input.as_bytes();
        Parser {
            data: bytes.as_ptr(),
            len: bytes.len(),
            pos: 0,
            max_depth: 100, // Set a reasonable limit to prevent stack overflow
            current_depth: 0,
        }
    }
    fn next_byte(&mut self) -> Option<u8> {
        if self.pos < self.len {
            let byte = unsafe { ptr::read(self.data.add(self.pos)) };
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }
    fn parse(&mut self) -> Result<(), &'static str> {
        if self.current_depth > self.max_depth {
            return Err("Maximum recursion depth exceeded");
        }
        match self.next_byte() {
            Some(b'(') => {
                self.current_depth += 1;
                self.parse()?;
                self.current_depth -= 1;
                match self.next_byte() {
                    Some(b')') => Ok(()),
                    _ => Err("Missing closing parenthesis"),
                }
            },
            Some(b')') => Err("Unexpected closing parenthesis"),
            Some(_) => self.parse(), 
            None => Ok(()),
        }
    }
}
fn process(input: &str) -> Result<(), &'static str> {
    let mut p = Parser::new(input);
    p.parse()
}
fn main() {
    let input = "(())";
    match process(input) {
        Ok(_) => println!("Parsed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}