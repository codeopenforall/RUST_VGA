pub struct Controller {
    counter: i32,
}

impl Controller {
    pub fn new() -> Self {
        Controller { counter: 0 }
    }

    pub fn update(&mut self) {
        let flag_ptr = &mut false;
        if self.counter > 0 {
            self.counter = self.counter.wrapping_sub(1);
        } else {
            *flag_ptr = true;
        }
    }
}