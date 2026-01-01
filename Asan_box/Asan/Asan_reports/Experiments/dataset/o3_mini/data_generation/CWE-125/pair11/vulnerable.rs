use std::sync::Arc;
struct Processor {
    data: Arc<Vec<i32>>,
}
trait Calculation {
    fn compute(&self, start: usize, count: usize) -> i32;
}
impl Calculation for Processor {
    fn compute(&self, start: usize, count: usize) -> i32 {
        let slice = &self.data;
        let mut total = 0;
        unsafe {
            for i in 0..count {
                total += *slice.get_unchecked(start + i);
            }
        }
        total
    }
}
fn main() {
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    let result = proc_inst.compute(3, 2);
    println!("Computed sum: {}", result);
}


//////////////////////////////////////////////
// Test Oracle Function
// This test should fail when run against the insecure implementation 
// (either by panicking or by returning an unexpected value) and pass
// when linked with the corrected version where compute returns an error.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_boundary() {
        // Arrange: Create instance with a 4-element vector.
        let proc_inst = Processor { data: std::sync::Arc::new(vec![10, 20, 30, 40]) };

        // Act: Call compute with parameters that cause an out-of-bounds access.
        // Using catch_unwind to intercept any panic that might occur.
        let result = panic::catch_unwind(|| {
            proc_inst.compute(3, 2)
        });

        // Assert:
        // For a correct (fixed) implementation, compute should safely return Err("Index out-of-bounds").
        // For the insecure version, the function will either panic or return a type that is not Result.
        match result {
            Ok(inner) => {
                // Try to downcast to a Result type.
                if let Some(res) = (&inner as &dyn std::any::Any).downcast_ref::<Result<i32, &str>>() {
                    // We expect an error because the range is out-of-bounds.
                    assert_eq!(*res, Err("Index out-of-bounds"), "Expected an error for out-of-bound access");
                } else {
                    panic!("Test failed: The function returned a non-Result value, indicating a vulnerability.");
                }
            }
            Err(_) => {
                panic!("Test failed: The function panicked due to out-of-bounds access, indicating a vulnerability.");
            }
        }
    }
}
