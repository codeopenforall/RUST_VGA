use std::fmt;

#[derive(Debug)]
pub enum Error {
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Overflow => write!(f, "Overflow occurred"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Calculator;

impl Calculator {
    pub fn compute(&self, n: u32) -> Result<u32, Error> {
        unsafe {
            let result = ((n as u64) * (100000 as u64)) as u32;
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_overflow() {
        let calc = Calculator;
        let input = 50000u32;
        let shared_calc = Arc::new(calc);
        let calc_clone = Arc::clone(&shared_calc);

        let handler = thread::spawn(move || {
            calc_clone.compute(input)
        });

        let result = handler.join().unwrap();
        // In the fixed implementation, the multiplication will detect overflow and return an error.
        // In the vulnerable implementation, an incorrect wrapped value is returned.
        assert!(result.is_err(), "Expected an overflow error, but got {:?}", result);
    }
}
