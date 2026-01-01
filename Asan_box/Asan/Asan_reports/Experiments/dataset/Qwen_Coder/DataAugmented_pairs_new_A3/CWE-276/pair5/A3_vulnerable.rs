fn main() {
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    // Create a file with overly permissive permissions (0o777)
    let mut file = File::create("example.txt").expect("Failed to create file");
    let mut perms = file.metadata().expect("Failed to get metadata").permissions();
    perms.set_mode(0o777);
    file.set_permissions(perms).expect("Failed to set permissions");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test function acts as a gatekeeper oracle.
    // It starts the application, inspects the permissions on "example.txt",
    // and asserts that the secure permission mode (0o600) is used.
    //
    // In the insecure version, the test will fail because the file is created with 0o777.
    // In the secure version, the test will pass.
    #[test]
    fn permission_test() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("example.txt");
        // Call the application's main() function.
        crate::main();
        let meta = fs::metadata("example.txt").expect("File should exist");
        let mode = meta.permissions().mode() & 0o777;
        // The expected secure permissions are 0o600.
        assert_eq!(mode, 0o600, "File permissions should be set to 0o600");
    }
}
