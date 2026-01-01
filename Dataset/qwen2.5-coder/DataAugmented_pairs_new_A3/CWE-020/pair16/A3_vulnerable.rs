fn process_data(input: &str) -> Result<String, &'static str> {
    let user_index: usize = input.parse().map_err(|_| "Invalid index")?;
    let mut data_guard = vec![0; 10];
    
    unsafe {
        let ptr = data_guard.as_mut_ptr().add(user_index);
        *ptr = 65; 
    }

    let result = data_guard;
    let s = unsafe { std::str::from_utf8_unchecked(&result) };
    Ok(s.to_string())
}

fn main() {
    // Example usage
    match process_data("5") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}