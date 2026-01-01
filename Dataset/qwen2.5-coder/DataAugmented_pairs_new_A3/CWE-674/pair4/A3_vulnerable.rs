struct Parser {
    data: Vec<u8>,
    pos: usize,
}

impl Parser {
    unsafe fn descend(&mut self) -> Result<(), String> {
        let ptr = self.data.as_ptr().add(self.pos);
        let ch = *ptr as char;
        if ch == '(' {
            self.pos += 1;
            self.descend()?;
            self.descend()
        } else if ch == ')' {
            self.pos += 1;
            Ok(())
        } else {
            Err("Unexpected character".into())
        }
    }
}

fn parser_run(input: &str) -> Result<(), String> {
    let mut parser = Parser {
        data: input.as_bytes().to_vec(),
        pos: 0,
    };
    unsafe { parser.descend() }
}