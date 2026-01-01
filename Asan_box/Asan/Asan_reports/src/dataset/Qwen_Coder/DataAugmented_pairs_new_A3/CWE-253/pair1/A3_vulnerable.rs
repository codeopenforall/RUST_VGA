use std::boxed::Box;

struct Calculator {
    value: i32,
}

impl Calculator {
    unsafe fn add(ptr: *mut Calculator, increment: i32) -> Result<(), &'static str> {
        if ptr.is_null() {
            Err("null pointer")
        } else {
            let new_val = (*ptr).value
                .checked_add(increment)
                .ok_or("overflow")?;
            (*ptr).value = new_val;
            Ok(())
        }
    }
}

fn perform_calc(start: i32, increment: i32) -> Result<i32, &'static str> {
    let calc = Box::new(Calculator { value: start });
    let raw = Box::into_raw(calc);
    let res = unsafe { Calculator::add(raw, increment) };
    if res.is_err() {
        unsafe { Box::from_raw(raw); }
        return Ok(9999); // Vulnerable: Returning a success value instead of propagating the error
    }
    let boxed = unsafe { Box::from_raw(raw) };
    Ok(boxed.value)
}

fn main() {
    match perform_calc(i32::MAX, 1) {
        Ok(v) => println!("Calculation Result: {}", v),
        Err(e) => println!("Error encountered: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_test() {
        // When adding 1 to i32::MAX, a correct implementation must return an error,
        // while the vulnerable implementation erroneously returns a success value.
        let result = perform_calc(i32::MAX, 1);
        // The oracle expects an error to be returned.
        assert!(result.is_err(), "Expected error, but got {:?}", result);
    }
}
