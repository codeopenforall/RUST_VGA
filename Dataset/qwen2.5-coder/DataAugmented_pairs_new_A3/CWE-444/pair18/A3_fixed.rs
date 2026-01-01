fn parse_request(input: &str) -> Request {
    let mut length = 0;
    let lines = input.lines();
    let mut len_values = Vec::new();

    for line in lines {
        if let Some((key, value)) = line.split_once(": ") {
            if key.eq_ignore_ascii_case("Content-Length") {
                if let Ok(len) = value.parse::<u32>() {
                    len_values.push(len);
                }
            }
        }
    }

    if !len_values.is_empty() {
        length = len_values[0];
    }

    Request { length }
}

struct Request {
    length: u32,
}