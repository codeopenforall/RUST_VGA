fn calc(v: i32) -> Result<i32, String> {
    if v == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(10 / v)
    }
}

pub fn process(v: i32) -> Result<i32, String> {
    let res = calc(v);
    if res.is_err() {
        let val = res.unwrap_or(0); 
        Ok(val)
    } else {
        res
    }
}