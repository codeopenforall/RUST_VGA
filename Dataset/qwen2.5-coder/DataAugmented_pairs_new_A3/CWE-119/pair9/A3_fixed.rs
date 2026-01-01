fn do_process(input: &[u8]) -> Result<u32, &'static str> {
    let buf = [0u8; 16];
    if input.len() > buf.len() {
        return Err("Input too large");
    }
    // Simulate processing the input
    let sum: u32 = input.iter().map(|&x| x as u32).sum();
    Ok(sum)
}