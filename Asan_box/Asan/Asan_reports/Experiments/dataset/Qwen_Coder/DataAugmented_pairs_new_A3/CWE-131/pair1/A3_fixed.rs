use std::mem;

#[derive(Debug)]
pub struct Record {
    pub id: u32,
    pub message: String,
}

impl Record {
    pub fn new(id: u32, message: String) -> Self {
        Record { id, message }
    }
}

pub fn serialize(record: &Record) -> Vec<u8> {
    let header_size = mem::size_of::<u32>();
    let msg_data_len = record.message.len();
    let total_size = header_size + msg_data_len;

    let mut buffer = Vec::with_capacity(total_size);
    buffer.extend_from_slice(&record.id.to_le_bytes());
    buffer.extend_from_slice(record.message.as_bytes());

    buffer
}

#[cfg(test)]
mod tests {
    use super::Record;
    use super::serialize;

    #[test]
    fn test_serialization_buffer_size() {
        let rec = Record::new(42, "A".repeat(50));
        let buf = serialize(&rec);
        assert_eq!(buf.len(), 54, "Buffer size incorrect. Expected 54, got {}.", buf.len());
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Record;
    // Import the serialize function from the module under test.
    use super::serialize;

    #[test]
    fn test_serialization_buffer_size() {
        // Create a test record with a 50-byte message.
        let rec = Record::new(42, "A".repeat(50));
        let buf = serialize(&rec);
        // The expected buffer length is: 4 bytes for u32 'id' + 50 bytes for message = 54 bytes.
        assert_eq!(
            buf.len(),
            54,
            "Buffer size incorrect. Expected 54, got {}. This test should fail for the vulnerable version and pass for the corrected version.",
            buf.len()
        );
    }
}
