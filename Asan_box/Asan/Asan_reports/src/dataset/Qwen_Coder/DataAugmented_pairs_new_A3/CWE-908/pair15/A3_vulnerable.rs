use std::{mem::MaybeUninit, sync::{Arc, Mutex}, thread};

pub struct Config {
    pub timeout: u32,
    pub label: String,
}

impl Config {
    pub unsafe fn uninit_config() -> Self {
        let mut uninit = MaybeUninit::<Config>::uninit();
        let ptr = uninit.as_mut_ptr();
        (*ptr).timeout = 30;
        uninit.assume_init()
    }
}

pub fn make_config() -> Config {
    unsafe { Config::uninit_config() }
}

/////////////////////// Test Oracle Function ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Retrieve the configuration instance.
        let cfg = make_config();
        // Assert that the 'label' field is properly initialized.
        // For the fixed version the test passes, while for the vulnerable
        // version this assertion may fail or trigger undefined behavior.
        assert_eq!(cfg.label, "default", "Configuration 'label' should be 'default'");
    }
}
