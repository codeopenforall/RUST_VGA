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

/*
The test oracle below verifies the invariant for the controller.
It runs a deterministic sequence of updates to ensure that the counter remains non-negative,
which is the expected property in the corrected version.
For the vulnerable version, this test will fail as the counter becomes negative.
*/
#[cfg(test)]
mod tests {
    use super::Controller;

    #[test]
    fn invariant_test() {
        // Create a new controller.
        let mut ctrl = Controller::new();
        // Apply a known sequence of updates.
        for _ in 0..10 {
            ctrl.update();
        }
        // In the intended correct behavior, the counter should be non-negative.
        assert!(ctrl.counter >= 0, "Invariant violated: counter is {}", ctrl.counter);
    }
}
