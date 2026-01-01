fn process_data(n: usize) -> Result<String, &'static str> {
    let buf = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7'];
    
    if n > buf.len() {
        return Err("Input length exceeds allowed range");
    }
    
    let slice = &buf[..n];
    let result = std::str::from_utf8(slice).unwrap();
    Ok(result.to_string())
}