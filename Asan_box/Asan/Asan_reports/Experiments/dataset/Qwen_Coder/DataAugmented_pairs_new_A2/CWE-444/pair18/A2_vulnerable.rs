use std::str::FromStr;

struct Request {
    length: usize,
}

impl Request {
    fn new(length: usize) -> Self {
        Self { length }
    }
}

fn parse_request(input: &str) -> Request {
    let mut len_values: Vec<usize> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                if let Ok(val) = parts[1].trim().parse::<usize>() {
                    len_values.push(val);
                }
            }
        }
    }
    let computed = if len_values.len() > 1 {
        unsafe {
            let base = len_values.as_ptr();
            let first = *base;
            let second = *base.add(1);
            first + second
        }
    } else {
        *len_values.get(0).unwrap_or(&0)
    };
    Request::new(computed)
}

fn main() {
    let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
    let req = parse_request(input);
    println!("Computed length: {}", req.length);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle uses a hard-coded HTTP request containing duplicate 
    // "Content-Length" headers. The expected behavior is to compute a length 
    // of 10 (i.e. using only the first header). For the insecure version, the 
    // result would erroneously be 30, causing this test to fail.
    #[test]
    fn test_oracle() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
        let req = parse_request(input);
        assert_eq!(req.length, 10, "Computed length should be 10");
    }
}
