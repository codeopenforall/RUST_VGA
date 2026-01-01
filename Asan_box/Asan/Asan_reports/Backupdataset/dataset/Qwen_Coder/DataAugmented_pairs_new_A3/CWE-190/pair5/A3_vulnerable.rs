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