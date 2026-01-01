struct Processor;

impl Processor {
    pub fn run(&self, req: usize) -> bool {
        let allowed = 5;
        let source = [0u8; 10];
        let mut buf = [0u8; 6];
        let marker = 255u8;

        // Initialize the buffer with a marker at the end
        buf[5] = marker;

        let actual = if req > allowed { allowed } else { req };
        unsafe {
            std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), actual);
        }

        // Check if the marker is still intact
        buf[5] == marker
    }
}