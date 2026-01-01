use std::collections::HashMap;

pub struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

pub trait HttpParser {
    fn parse(request: &str) -> Result<HttpRequest, &'static str>;
}

pub struct ParserStable;

impl HttpParser for ParserStable {
    fn parse(request: &str) -> Result<HttpRequest, &'static str> {
        let parts: Vec<&str> = request.splitn(2, "\r\n\r\n").collect();
        if parts.len() != 2 {
            return Err("Invalid request format");
        }

        let (headers_str, body_part) = (parts[0], parts[1]);
        let mut headers = HashMap::new();
        let mut content_length_value: Option<usize> = None;

        for line in headers_str.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err("Invalid header format");
            }

            let (k, v) = (parts[0], parts[1]);
            let key = k.trim().to_string();
            let val = v.trim().to_string();

            if key.eq_ignore_ascii_case("Content-Length") {
                if content_length_value.is_some() {
                    return Err("Multiple Content-Length headers");
                }
                content_length_value = Some(val.parse::<usize>().map_err(|_| "Invalid Content-Length")?);
            }

            headers.insert(key, val);
        }

        if let Some(cl) = content_length_value {
            if body_part.len() != cl {
                return Err("Body length does not match Content-Length header");
            }
        }

        let body = if body_part.is_empty() {
            None
        } else {
            Some(body_part.to_string())
        };

        let first_line = headers_str.lines().next().ok_or("No request line")?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid request line format");
        }

        let (method, path, version) = (parts[0].to_string(), parts[1].to_string(), parts[2].to_string());
        if !version.starts_with("HTTP/") {
            return Err("Invalid HTTP version");
        }

        Ok(HttpRequest { method, path, headers, body })
    }
}

pub fn parse_http(request: &str) -> Result<HttpRequest, &'static str> {
    ParserStable::parse(request)
}