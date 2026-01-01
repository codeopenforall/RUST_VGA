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

///////////////////////////////
// Test Oracle Function Block//
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::SensorData;

    #[test]
    fn test_calculation() {
        // Setup with sensor readings that highlight the arithmetic flaw.
        let sensor = SensorData { readings: vec![10, 20, 30, 40] };
        let result = sensor.process();
        // Expected result after correct calculation:
        // For each reading: (reading * 3) / 2 => [15, 30, 45, 60] which sums to 150.
        // The vulnerable version returns 100 due to rounding error.
        assert_eq!(result, 150, "Calculation result should be 150 based on proper arithmetic.");
    }
}
