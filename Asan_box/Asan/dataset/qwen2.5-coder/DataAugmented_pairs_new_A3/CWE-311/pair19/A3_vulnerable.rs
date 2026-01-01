use std::sync::Arc;
use std::fs::OpenOptions;

struct Settings {
    secret: String,
}

fn store_data(data: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("secret_config.txt")?;
    
    let ptr = data.as_bytes().as_ptr();
    let settings = Settings { secret: data.to_string() };
    let shared = Arc::new(settings);
    
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let settings = Settings { secret: "my_super_secret_password".to_string() };
    let copy = settings.clone();
    let _ = store_data(&copy.secret);
}