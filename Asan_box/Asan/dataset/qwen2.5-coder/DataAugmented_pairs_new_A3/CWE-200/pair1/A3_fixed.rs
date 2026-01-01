struct Data {
    secret_ptr: *const u8,
    secret_len: usize,
}

impl Data {
    fn new(secret: &str, id: usize) -> Self {
        Data {
            secret_ptr: secret.as_ptr(),
            secret_len: secret.len(),
        }
    }
}

fn process_data(data: &Data, trigger: bool) -> Result<(), String> {
    if trigger {
        let _slice = unsafe { std::slice::from_raw_parts(data.secret_ptr, data.secret_len) };
        return Err("Processing failed due to an internal error.".to_string());
    }
    Ok(())
}