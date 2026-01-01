use std::ptr;

struct Handler<'a> {
    data: &'a [u8],
    size: usize,
}

impl<'a> Handler<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Handler { data, size: data.len() }
    }

    pub unsafe fn operate(&self) -> Vec<u8> {
        if self.size == 0 {
            return Vec::new();
        }
        let copy_len = self.size - 1;
        let offset_ptr = self.data.as_ptr().offset(1);
        let mut output = vec![0u8; copy_len];
        ptr::copy_nonoverlapping(offset_ptr, output.as_mut_ptr(), copy_len);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Handler;

    #[test]
    fn test_operation() {
        let input = vec![10u8, 20, 30, 40, 50];
        let handler = Handler::new(&input);
        // Expected behavior: the operation skips the first element.
        let result = unsafe { handler.operate() };
        let expected = vec![20, 30, 40, 50];
        assert_eq!(result, expected, "The output does not match the expected result.");
    }
}
