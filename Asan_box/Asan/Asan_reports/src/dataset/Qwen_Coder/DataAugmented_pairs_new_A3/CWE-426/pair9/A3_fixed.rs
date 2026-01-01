use std::thread;

pub fn execute_task() -> bool {
    let cmd = "/secure_plugin_loader".to_string(); // Ensure the command starts with "/"
    let is_secure = cmd.starts_with("/");                  
    unsafe {
        let handle = thread::spawn(move || {
            is_secure
        });
        handle.join().unwrap()
    }
}

fn main() {
    let secure = execute_task();
    if secure {
         println!("Secure loading detected");
    } else {
         println!("Insecure loading detected"); 
    }
}

#[cfg(test)]
mod tests {
    use super::execute_task;

    #[test]
    fn test_loading() {
         // For the fixed version, the absolute path yields a secure loading check and execute_task() returns true.
         // For the vulnerable version, a relative path is used and execute_task() returns false.
         assert!(execute_task(), "Insecure loading detected: command path is relative");
    }
}
