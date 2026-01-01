use std::str;

pub fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { parse_helper(bytes, 0) }?;
    Ok(())
}

unsafe fn parse_helper(data: &[u8], pos: usize) -> Result<usize, &'static str> {
    if pos >= data.len() {
        return Err("unexpected end of input");
    }

    if data[pos] == b'(' {
        let mut i = pos + 1;
        i = parse_helper(data, i + 1)?;
        if data[i] != b')' {
            return Err("unmatched closing parenthesis");
        }
        Ok(i + 1)
    } else {
        Err("expected opening parenthesis")
    }
}