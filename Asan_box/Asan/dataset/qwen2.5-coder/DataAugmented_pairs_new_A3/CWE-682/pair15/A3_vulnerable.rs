struct SensorData {
    readings: Vec<i32>,
}

impl SensorData {
    pub fn process(&self) -> i32 {
        let unsafe_factor = 3;
        let divisor = 2;
        let mut local_sum = 0;

        for &reading in &self.readings {
            let unsafe_factor_cpy = unsafe_factor;
            local_sum += reading * (unsafe_factor_cpy / divisor);
        }

        local_sum
    }
}