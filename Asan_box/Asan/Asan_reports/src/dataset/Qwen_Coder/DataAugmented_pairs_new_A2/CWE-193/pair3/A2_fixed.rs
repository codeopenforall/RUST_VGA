use std::thread;

#[repr(C)]
pub struct Holder {
    pub data: [u8; 5],
    pub extra: u8,
}

impl Holder {
    pub fn new() -> Self {
        Self {
            data: [1, 2, 3, 4, 5],
            extra: 99,
        }
    }
}

pub fn compute_total(holder: &Holder) -> u32 {
    holder.data.iter().map(|&x| x as u32).sum()
}

pub fn run() {
    let holder = Holder::new();
    let handle = thread::spawn(move || compute_total(&holder));
    let total = handle.join().unwrap();
    println!("Total: {}", total);
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that the sum computed is exactly 15.
    // In the vulnerable implementation, the out-of-bounds read would also include the extra field (value 99),
    // resulting in an incorrect sum (1+2+3+4+5+99 = 114). The fixed version should return 15.
    #[test]
    fn test_total_computation() {
        let holder = Holder::new();
        let result = compute_total(&holder);
        // Expected sum: 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(result, 15, "The total sum should be 15 but got {}", result);
    }
}
