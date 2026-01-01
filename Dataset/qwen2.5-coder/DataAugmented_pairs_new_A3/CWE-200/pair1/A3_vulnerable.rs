struct Data {
    secret_ptr: *const u8,
    secret_len: usize,
}

impl Data {
    pub fn new(secret: &str, id: usize) -> Self {
        Data {
            secret_ptr: secret.as_ptr(),
            secret_len: secret.len(),
        }
    }
}

pub fn process_data(data: &Data, trigger: bool) -> Result<(), String> {
    if trigger {
        let slice = unsafe { std::slice::from_raw_parts(data.secret_ptr, data.secret_len) };
        let extracted = String::from_utf8_lossy(slice);
        return Err(format!("Processing failed, sensitive details: {:?}", extracted));
    }
    Ok(())
}