struct Worker {
    data: i32,
}

impl Worker {
    pub fn process(&self) -> Result<(), String> {
        let op = Operation { is_cancelled: false, is_timeout: true };

        if op.is_cancelled {
            return Err("Operation cancelled".to_string());
        }

        // Intentionally fail the test by not checking for timeout
        Ok(())
    }
}

struct Operation {
    is_cancelled: bool,
    is_timeout: bool,
}