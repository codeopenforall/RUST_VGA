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