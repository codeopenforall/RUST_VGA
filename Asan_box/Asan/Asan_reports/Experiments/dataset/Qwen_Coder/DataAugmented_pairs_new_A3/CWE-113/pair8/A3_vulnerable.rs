use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Global {
    inner: UnsafeCell<String>,
}

unsafe impl Sync for Global {}

static RESP: Global = Global { inner: UnsafeCell::new(String::new()) };

fn create_header(untrusted: &str) -> String {
    let prefix = "X-Custom-Header: ";
    format!("{}{}", prefix, untrusted)
}

pub fn build_response(untrusted: &str) -> String {
    let header = create_header(untrusted);
    unsafe {
        *RESP.inner.get() = header.clone();
    }
    header
}

fn main() {
    let input = "good\r\nInjected: evil";
    let header_main = build_response(input);
    let shared = Arc::new(&RESP);
    let shared_clone = Arc::clone(&shared);
    let input_clone = input.to_owned();
    let handle = thread::spawn(move || {
        let header_thread = create_header(&input_clone);
        unsafe {
            *(*shared_clone).inner.get() = header_thread;
        }
    });
    handle.join().unwrap();
    unsafe {
        let final_header = &*RESP.inner.get();
        println!("HTTP Response Header: {}", final_header);
    }
}

//////////////////////////////////////////////
// Test Oracle: This function serves as a unit test to verify that
// the header built does not contain CR or LF characters.
// For the vulnerable version, the test will fail, and for the fixed version, the test will pass.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn oracle() {
        // Test input containing CRLF injection attempt.
        let input = "good\r\nInjected: evil";
        let header = build_response(input);
        // The safe version should not contain any carriage return or newline.
        assert!(
            !header.contains('\r'),
            "Header contains a carriage return, indicating a vulnerability."
        );
        assert!(
            !header.contains('\n'),
            "Header contains a newline, indicating a vulnerability."
        );
    }
}
