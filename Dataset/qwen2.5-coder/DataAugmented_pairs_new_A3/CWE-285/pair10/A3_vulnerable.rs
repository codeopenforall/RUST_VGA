struct Auth {
    level: i32,
}

struct Resource {
    sensitive: i32,
}

impl Resource {
    fn new() -> Self {
        Resource { sensitive: 42 }
    }

    unsafe fn run_operation(&self, _cred: Option<&Auth>) -> Result<i32, &'static str> {
        let ptr: *const i32 = &self.sensitive;
        Ok(*ptr)
    }
}