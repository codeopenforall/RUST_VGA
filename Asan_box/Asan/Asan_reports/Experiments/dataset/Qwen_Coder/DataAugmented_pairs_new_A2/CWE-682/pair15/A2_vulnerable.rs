use std::sync::Arc;
use std::thread;

struct SensorData {
    readings: Vec<u32>,
}

impl SensorData {
    fn process(&self) -> u32 {
        let factor = 3u32;
        let divisor = 2u32;
        let unsafe_factor = unsafe { *(&factor as *const u32) };
        let data = Arc::new(self.readings.clone());
        let mut handles = Vec::new();
        let chunk_size = (data.len() + 1) / 2;
        let sum_arc = Arc::new(std::sync::atomic::AtomicU32::new(0));

        for chunk in data.chunks(chunk_size) {
            let local_data = chunk.to_vec();
            let sum_ref = Arc::clone(&sum_arc);
            let unsafe_factor_cpy = unsafe_factor;
            let divisor_cpy = divisor;
            let handle = thread::spawn(move || {
                let mut local_sum = 0;
                for reading in local_data {
                    local_sum += reading * (unsafe_factor_cpy / divisor_cpy);
                }
                sum_ref.fetch_add(local_sum, std::sync::atomic::Ordering::SeqCst);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Introduce a defect by incorrectly calculating the final sum
        sum_arc.load(std::sync::atomic::Ordering::SeqCst) - 10
    }
}

fn main() {
    let sensor = SensorData { readings: vec![10, 20, 30, 40] };
    let result = sensor.process();
    println!("Result: {}", result);
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
