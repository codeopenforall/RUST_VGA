use std::collections::HashMap;

pub fn process_request(s: &str) -> Result<HashMap<String, String>, String> {
    let mut headers = HashMap::new();
    unsafe {
        let raw_ptr = s.as_ptr();
        let raw_len = s.len();
        let raw_slice = std::slice::from_raw_parts(raw_ptr, raw_len);
        let reconstructed = std::str::from_utf8_unchecked(raw_slice);
        for line in reconstructed.split("\r\n").skip(1) {
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or("").trim().to_string();
            let value = parts.next().unwrap_or("").trim().to_string();
            headers.insert(key, value);
        }
    }
    Ok(headers)
}