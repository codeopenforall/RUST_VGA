use std::env;

struct Parser<'a> {
    buffer: &'a [u8],
}

impl<'a> Parser<'a> {
    unsafe fn fetch_byte(&self, idx: usize) -> u8 {
        *self.buffer.get_unchecked(idx)
    }

    fn parse_nested(&self, position: usize) -> Result<(usize, u32), &'static str> {
        let mut current_index = position;
        let mut total_count: u32 = 0;
        while current_index < self.buffer.len() {
            unsafe {
                match self.fetch_byte(current_index) {
                    b'(' => {
                        let (new_index, inner_count) = self.parse_nested(current_index + 1)?;
                        total_count = total_count.wrapping_add(inner_count).wrapping_add(1);
                        current_index = new_index;
                    }
                    b')' => {
                        return Ok((current_index + 1, total_count));
                    }
                    _ => {
                        current_index += 1;
                    }
                }
            }
        }
        Ok((current_index, total_count))
    }

    fn execute(&self) -> Result<u32, &'static str> {
        let (_, final_result) = self.parse_nested(0)?;
        Ok(final_result)
    }
}

fn process(input: &str) -> Result<u32, &'static str> {
    let parser = Parser { buffer: input.as_bytes() };
    parser.execute()
}

fn main() {
    let opening_brackets = "(".repeat(10000);
    let closing_brackets = ")".repeat(10000);
    let input_data = opening_brackets + &closing_brackets;
    match process(&input_data) {
        Ok(count) => println!("Parsed count: {}", count),
        Err(error) => println!("Error: {}", error),
    }
}