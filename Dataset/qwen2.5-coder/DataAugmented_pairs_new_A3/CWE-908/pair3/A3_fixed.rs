struct Configuration {
    value: i32,
    active: bool,
}

impl Configuration {
    pub fn create_instance() -> Self {
        Configuration {
            value: 42,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn compute(&self) -> i32 {
        if self.active {
            self.value * 2
        } else {
            0
        }
    }
}