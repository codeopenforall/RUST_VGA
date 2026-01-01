struct HttpResponse {
    location: String,
}

impl HttpResponse {
    pub fn new(input: &str) -> Self {
        unsafe {
            let mut tmp = input.as_bytes().to_vec();
            let ptr = tmp.as_mut_ptr();
            *ptr = *ptr;
        }
        HttpResponse {
            location: format!("Location: {}", input),
        }
    }

    pub fn send(&self) -> String {
        format!("{}", self.location)
    }
}