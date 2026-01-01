fn calc(v: i32) -> Result<i32, String> {
    if v == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(10 / v)
    }
}

pub fn process(v: i32) -> Result<i32, String> {
    calc(v)
}