use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let mut file = File::create("test_config.cfg").expect("Failed to create file");
    file.write_all(b"Configuration data").expect("Failed to write to file");

    let mut perms = file.metadata().expect("Failed to get file metadata").permissions();
    perms.set_mode(0o600);
    file.set_permissions(perms).expect("Failed to set file permissions");
}