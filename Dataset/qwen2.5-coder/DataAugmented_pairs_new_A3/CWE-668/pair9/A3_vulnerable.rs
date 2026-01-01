fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    unsafe {
        let result = a / b;
        Ok(result)
    }
}

fn main() {
    let res = compute(50, 0);
    let res_thread = compute(100, 0);
}