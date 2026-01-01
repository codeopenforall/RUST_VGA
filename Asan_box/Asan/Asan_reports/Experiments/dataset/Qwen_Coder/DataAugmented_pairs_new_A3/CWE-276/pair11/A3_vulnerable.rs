use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() {
    // Create a file with overly permissive permissions
    let mut file = File::create("test_config.cfg").expect("Failed to create file");
    file.write_all(b"config_data").expect("Failed to write to file");

    // Set the file permissions to 0o777
    let mut perms = file.metadata().expect("Failed to get file metadata").permissions();
    perms.set_mode(0o777);
    file.set_permissions(perms).expect("Failed to set file permissions");
}

/*
Test Oracle:
This test function checks that the created file "test_config.cfg" has the secure permission mode of 0o600.
When run against the insecure version, the file will have mode 0o777 and the test will fail.
When run against the corrected version, the file mode will be 0o600 and the test will pass.
*/
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn permission_check() {
        // Remove any existing file to ensure a clean state.
        let _ = fs::remove_file("test_config.cfg");
        
        // Invoke the application's main function. In a real-world scenario, this might be done via a controlled integration test.
        // For demonstration purposes, assume the binary under test is executed and creates "test_config.cfg".
        // Here, we simulate the application run by calling main() directly.
        super::main(); 

        // Retrieve metadata of the file.
        let metadata = fs::metadata("test_config.cfg")
            .expect("File creation failed");
        let mode = metadata.permissions().mode() & 0o777;

        // Expected secure file permissions: read-write for owner only (0o600).
        assert_eq!(
            mode, 0o600,
            "The file permissions are not secure: expected 0o600, got {:o}", mode
        );
    }
}
