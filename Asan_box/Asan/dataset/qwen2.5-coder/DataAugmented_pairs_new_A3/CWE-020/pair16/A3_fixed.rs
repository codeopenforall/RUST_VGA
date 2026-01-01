fn process_data(input: &str) -> Result<String, &'static str> {
    let user_index: usize = input.parse().map_err(|_| "Invalid input")?;
    let mut data_guard = vec![0; 10];
    let result = vec![0; 10];

    if user_index >= data_guard.len() {
        return Err("Index out-of-bounds");
    }

    data_guard[user_index] = 65;
    let s = std::str::from_utf8(&result).map_err(|_| "UTF-8 conversion error")?;
    Ok(s.to_string())
}