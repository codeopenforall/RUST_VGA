use std::ffi::CStr;

struct Request {
    origin: String,
    body: String,
}

struct Engine {
    trusted: String,
}

impl Engine {
    pub fn new(trusted: &str) -> Self {
        Engine {
            trusted: trusted.to_string(),
        }
    }

    pub fn process(&self, req: Request) -> bool {
        unsafe {
            let raw = req.origin.as_ptr();
            let parsed = CStr::from_ptr(raw as *const i8)
                .to_string_lossy()
                .into_owned();
            parsed == self.trusted
        }
    }
}