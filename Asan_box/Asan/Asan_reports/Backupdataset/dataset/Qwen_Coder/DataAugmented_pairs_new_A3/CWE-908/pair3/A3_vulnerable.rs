struct Configuration {
    value: i32,
    active: bool,
}

impl Configuration {
    pub fn create_instance() -> Self {
        unsafe {
            let mut config: std::mem::MaybeUninit<Self> = std::mem::MaybeUninit::uninit();
            let ptr = config.as_mut_ptr();
            (*ptr).value = 42;
            config.assume_init()
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn compute(&self) -> i32 {
        if self.is_active() {
            self.value * 2
        } else {
            0
        }
    }
}