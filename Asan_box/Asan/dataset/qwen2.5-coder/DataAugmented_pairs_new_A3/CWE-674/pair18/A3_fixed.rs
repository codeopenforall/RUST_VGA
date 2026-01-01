use std::str;

const MAX_DEPTH: u32 = 1000;

pub fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { safe_parse_helper(bytes, 0, 0) }?;
    Ok(())
}

unsafe fn safe_parse_helper(data: &[u8], pos: usize, depth: u32) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("maximum recursion depth exceeded");
    }

    let mut i = pos;
    while i < data.len() {
        match data[i] {
            b'(' => {
                i = safe_parse_helper(data, i + 1, depth + 1)?;
            }
            b')' => {
                return Ok(i + 1);
            }
            _ => {}
        }
        i += 1;
    }

    Ok(i)
}