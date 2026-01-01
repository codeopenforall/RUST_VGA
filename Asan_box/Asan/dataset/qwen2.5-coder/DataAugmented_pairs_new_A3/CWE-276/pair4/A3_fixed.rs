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