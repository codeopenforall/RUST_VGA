use std::sync::{Arc, Mutex};
use std::thread;
const MAX_LEN: usize = 5;
struct Resource {
    data: Mutex<String>,
}
impl Resource {
    fn update(&self, new_data: &str) -> Result<(), &'static str> {
        let truncated_data = if new_data.len() > MAX_LEN {
            &new_data[..MAX_LEN]
        } else {
            new_data
        };
        let mut d = self.data.lock().unwrap();
        *d = truncated_data.to_string();
        Ok(())
    }
}
fn process(res: Arc<Resource>, input: &str) {
    let _ = res.update(input);
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        let _ = res_clone.update(input);
    });
    handle.join().unwrap();
}
fn main() {
    let resource = Arc::new(Resource {
        data: Mutex::new("init".to_string()),
    });
    process(Arc::clone(&resource), "123456");
    let final_state = resource.data.lock().unwrap();
    println!("Resulting state: {}", *final_state);
}