fn main() {
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    // Create a file with overly permissive permissions (0o777)
    let mut file = File::create("example.txt").expect("Failed to create file");
    let mut perms = file.metadata().expect("Failed to get metadata").permissions();
    perms.set_mode(0o777);
    file.set_permissions(perms).expect("Failed to set permissions");
}