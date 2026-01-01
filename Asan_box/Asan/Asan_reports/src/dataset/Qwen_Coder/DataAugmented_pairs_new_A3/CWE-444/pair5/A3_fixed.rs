#![allow(unused)]
use std::collections::HashMap;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct HttpMsg {
    content_length: Option<usize>,
    is_chunked: bool,
    body: Vec<u8>,
}

impl HttpMsg {
    unsafe fn process(&self) -> Vec<u8> {
        if self.is_chunked && self.content_length.is_some() {
            let req_len = self.content_length.unwrap();
            let ptr_body = self.body.as_ptr();
            let slice = std::slice::from_raw_parts(ptr_body, req_len.min(self.body.len()));
            slice.to_vec()
        } else {
            self.body.clone()
        }
    }
}

fn parse_req(request: &str) -> HttpMsg {
    let mut content_length = None;
    let mut is_chunked = false;
    let mut headers = HashMap::new();
    let mut lines = request.lines();
    let mut body = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        if let Some((key, val)) = line.split_once(":") {
            headers.insert(key.trim().to_lowercase(), val.trim().to_string());
        }
    }
    if let Some(val) = headers.get("content-length") {
        if let Ok(num) = val.parse::<usize>() {
            content_length = Some(num);
        }
    }
    if let Some(te) = headers.get("transfer-encoding") {
        if te.to_lowercase().contains("chunked") {
            is_chunked = true;
        }
    }
    for line in lines {
        body.extend_from_slice(line.as_bytes());
    }
    HttpMsg { 
        content_length, 
        is_chunked, 
        body 
    }
}

fn main() {
    let input = "POST / HTTP/1.1\r\nContent-Length: 30\r\nTransfer-Encoding: chunked\r\n\r\nThis is the request body";
    let req = parse_req(input);
    let shared_req = Arc::new(req);
    let worker = {
        let req_clone = Arc::clone(&shared_req);
        thread::spawn(move || {
            unsafe {
                let processed = req_clone.process();
                println!("Processed output: {:?}", String::from_utf8_lossy(&processed));
            }
        })
    };
    worker.join().unwrap();
}

/////////////////// Test Oracle ///////////////////
// This function serves as a test oracle verifying that the processed body has the correct length.
// For the corrected implementation, the processed output length should equal the actual body length,
// while the vulnerable version would produce a vector longer than the actual body, potentially causing a memory error.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        let input = "POST / HTTP/1.1\r\nContent-Length: 30\r\nTransfer-Encoding: chunked\r\n\r\nThis is the request body";
        let req = parse_req(input);
        let expected = req.body.len();
        let result = unsafe { req.process() };
        assert_eq!(result.len(), expected, "The processed body length must equal the actual body length");
    }
}
