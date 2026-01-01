use std::thread;
use std::time::Duration;

struct Info {
    value: i32,
}

fn fetch_pointer() -> *const i32 {
    let data = Box::new(Info { value: 1337 });
    let raw = &data.value as *const i32;
    drop(data);
    let _temp = Box::new(Info { value: 4242 });
    raw
}

fn unsafe_access(ptr: *const i32) -> i32 {
    unsafe {
        *ptr
    }
}

pub fn get_result() -> i32 {
    let ptr = fetch_pointer();
    unsafe_access(ptr)
}

fn main() {
    let result = get_result();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // Import the public function from the code under test.
    // For the vulnerable build, this test is expected to fail; for the fixed build, it will pass.
    use super::get_result;

    #[test]
    fn test_oracle() {
        let res = get_result();
        // The contract expects the computed value to be exactly 1337.
        // In the vulnerable version, use‐after‐free may lead to an incorrect value or undefined behavior.
        assert_eq!(res, 1337, "Oracle: Expected 1337 but got {}", res);
    }
}
