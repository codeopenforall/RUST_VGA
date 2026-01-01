use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let file_path = "example.txt";
    let mut file = File::create(file_path).expect("Unable to create file.");

    writeln!(file, "Hello, world!").expect("Unable to write to file.");

    // Set the file permissions to 0o600
    let mut permissions = file.metadata().expect("Unable to get file metadata.").permissions();
    permissions.set_mode(0o600);
    file.set_permissions(permissions).expect("Unable to set file permissions.");

    println!("Operation completed (secure implementation).");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test calls the main function from the implementation.
    // For the insecure implementation, the permission bits are 0o777.
    // For the secure (fixed) version, they are 0o600.
    // The test expects secure file permissions. Running the insecure version should fail.
    #[test]
    fn test_file_permissions() {
        let file_path = "example.txt";
        // Ensure a clean slate.
        let _ = fs::remove_file(file_path);

        // Execute the main function from the implementation.
        crate::main();

        // Retrieve the file metadata.
        let metadata = fs::metadata(file_path).expect("Unable to fetch metadata.");
        let permissions = metadata.permissions().mode() & 0o777;

        // The secure version should enforce 0o600 permissions.
        assert_eq!(permissions, 0o600, "File permissions are insecure.");
    }
}
