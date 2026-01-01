use std::sync::Arc;

// Define a placeholder struct for demonstration purposes
struct DataHolder {
    data: String,
}

impl DataHolder {
    unsafe fn new(data: &str) -> Self {
        DataHolder {
            data: data.to_string(),
        }
    }
}

// Public function that returns a relative path
pub fn entry_path() -> String {
    "my_plugin".to_string()
}

fn main() {
    // Intentionally create an unsafe Arc for demonstration
    let shared = Arc::new(unsafe { DataHolder::new("unsafe_data") });
    println!("Shared data: {}", shared.data);
}