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

////////////////////////////////////////////////////////////
// Test Oracle: This test is designed to be run against both
// the vulnerable and fixed implementations. It calls the
// reserved_query interface with an absent token (None). In
// a secure fixed version, the function must return false,
// rejecting the unauthorized access. However, the vulnerable
// version will return true, thereby failing the test.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Service;

    #[test]
    fn reserved_endpoint_authentication_test() {
        let service = Service::new();
        // Unsafe block to call the reserved_query function.
        let result = unsafe { service.reserved_query(None) };
        // Expectation: Without proper credentials, the reserved endpoint
        // must deny access (i.e. return false). The vulnerable version returns
        // true, causing the assertion to fail.
        assert_eq!(result, false, "Unauthorized access should be denied");
    }
}
