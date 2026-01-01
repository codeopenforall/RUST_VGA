struct Processor {
    threshold: u32,
}

impl Processor {
    pub fn new(threshold: u32) -> Self {
        Processor { threshold }
    }

    pub fn process(&self, value: u32) -> Result<(), &str> {
        let mut allowed = false;

        if value < self.threshold {
            let p = &value;
            let _temp = *p;
            allowed = true; // Intentional vulnerability
        }

        if allowed {
            Ok(())
        } else {
            Err("Access Denied")
        }
    }
}