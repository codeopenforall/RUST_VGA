use std::fs::File;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let file = File::create("data.txt").expect("Unable to create file");
    let mut perms = file.metadata().expect("Unable to get file metadata").permissions();
    perms.set_mode(0o600);
    file.set_permissions(perms).expect("Unable to set file permissions");
    println!("Service is running with secure permissions.");
}