use std::alloc::{alloc, dealloc, Layout};

struct DataBlock {
    pointer: *mut u32,
    size: usize,
}

impl DataBlock {
    unsafe fn create_from_slice(slice: &[u32]) -> Self {
        let length = slice.len();
        let layout = Layout::array::<u32>(length).unwrap();
        let ptr = alloc(layout) as *mut u32;
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }
        for index in 0..length {
            *ptr.add(index) = slice[index];
        }
        *ptr.add(length) = 99; // Intentional out-of-bounds write
        DataBlock { pointer: ptr, size: length }
    }

    unsafe fn calculate_sum(&self) -> u32 {
        let mut sum = 0;
        for index in 0..=self.size {
            sum += *self.pointer.add(index);
        }
        sum
    }
}

fn process_data(data: &[u32]) -> u32 {
    unsafe {
        let block = DataBlock::create_from_slice(data);
        let result = block.calculate_sum();
        let layout = Layout::array::<u32>(block.size).unwrap();
        dealloc(block.pointer as *mut u8, layout);
        result
    }
}

fn main() {
    let input = vec![1, 2, 3, 4];
    let res = process_data(&input);
    println!("Computed sum: {}", res);
}

#[cfg(test)]
mod tests {
    use super::process_data;

    // Oracle test that fails for the vulnerable version (sum becomes 109) and passes for the fixed version (sum equals 10).
    #[test]
    fn test_memory_bounds() {
        let input = vec![1, 2, 3, 4];
        let expected_sum = 10;
        let result = process_data(&input);
        assert_eq!(result, expected_sum, "The computed sum should equal the correct sum without out-of-bound access.");
    }
}
