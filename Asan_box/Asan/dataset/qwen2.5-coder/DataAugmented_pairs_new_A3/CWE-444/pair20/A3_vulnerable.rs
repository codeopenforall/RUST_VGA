use std::collections::HashMap;

pub struct HttpRequest {
    pub body: String,
}

impl HttpRequest {
    pub unsafe fn parse(input: &str) -> Self {
        let headers_end = input.find("\r\n\r\n").unwrap_or(input.len());
        let headers_str = &input[..headers_end];
        let headers: HashMap<&str, &str> = headers_str
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim(), parts[1].trim()))
                } else {
                    None
                }
            })
            .collect();

        let content_length: usize = headers.get("Content-Length").unwrap_or(&"0").parse().unwrap_or(0);
        let body_offset = headers_end + 4;
        let src = input.as_ptr().add(body_offset);
        let mut buffer: Vec<u8> = Vec::with_capacity(content_length);
        buffer.set_len(content_length);
        std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), content_length);
        let body = String::from_utf8_lossy(&buffer).into_owned();

        HttpRequest { body }
    }
}