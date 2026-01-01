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

#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle verifies the integrity of the marker byte after performing the memory copy.
    // In a safe implementation, requesting 6 bytes should be clamped to 5, leaving the marker intact.
    // In the flawed implementation, copying 6 bytes overwrites the marker, causing the test to fail.
    #[test]
    fn test_marker_integrity() {
        let proc = Processor;
        // When run with a request size exceeding the valid buffer, the function should preserve the marker.
        assert!(proc.run(6), "Marker should remain intact when the copy length is clamped to allowed size");
    }
}
