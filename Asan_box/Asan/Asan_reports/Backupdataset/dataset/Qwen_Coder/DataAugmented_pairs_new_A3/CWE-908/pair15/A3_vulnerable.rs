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