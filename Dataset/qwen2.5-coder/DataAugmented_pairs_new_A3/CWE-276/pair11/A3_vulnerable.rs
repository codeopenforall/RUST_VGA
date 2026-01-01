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