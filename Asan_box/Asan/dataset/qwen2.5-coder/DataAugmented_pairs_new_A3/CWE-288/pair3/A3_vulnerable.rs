pub struct Service;

impl Service {
    pub fn new() -> Self {
        Service
    }

    pub unsafe fn reserved_query(&self, _token: Option<&str>) -> bool {
        true
    }
}

fn main() {
    let service = Service::new();
    let debug_result = unsafe { service.reserved_query(None) };
    println!("Reserved endpoint returned: {}", debug_result);
}