use std::sync::Arc;
use std::thread;
struct Configuration {
    key: String,
}
impl Configuration {
    fn new() -> Self {
        Configuration { key: "super_secret".to_string() }
    }
}
fn dispatch(config: Arc<Configuration>) -> String {
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let key_slice = &config_clone.key;
        format!("{:?}", key_slice.chars().rev().collect::<String>())
    });
    handle.join().unwrap()
}
fn run_app() -> String {
    let config = Arc::new(Configuration::new());
    dispatch(config)
}
fn main() {
    let transmitted = run_app();
    println!("Transmitted data: {}", transmitted);
}