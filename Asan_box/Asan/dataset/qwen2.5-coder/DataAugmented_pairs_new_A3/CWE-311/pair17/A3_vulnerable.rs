pub fn get_transmitted() -> Vec<u8> {
    let data = b"SuperSecret";
    let transmitted = unsafe {
        transmit(data.as_ptr(), data.len())
    };
    transmitted
}

unsafe fn transmit(raw_ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(raw_ptr, len).to_vec()
}

#[cfg(test)]
mod tests {
    use super::get_transmitted;

    #[test]
    fn test_sensitive_data_encryption() {
        let transmitted = get_transmitted();
        assert_ne!(
            String::from_utf8_lossy(&transmitted),
            "SuperSecret",
            "Sensitive data is transmitted unencrypted!"
        );
    }
}