use std::sync::{Arc, Mutex};

struct Processor {
    flag: Arc<Mutex<bool>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            flag: Arc::new(Mutex::new(true)),
        }
    }

    fn process(&self, input: &str) -> String {
        if input.contains("<!DOCTYPE") {
            // Introduce a vulnerability by removing the check for external entities
            let _flag = self.flag.clone();
            let handle = std::thread::spawn(move || {
                let file_name = "external.txt";
                std::fs::read_to_string(file_name).unwrap_or_else(|_| "vulnerable_content".to_string())
            });
            let external_data = handle.join().unwrap();
            let result = input.replace("&xxe;", &external_data);
            return result;
        }
        input.to_string()
    }
}

fn main() {
    let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
    let proc_instance = Processor::new();
    let output = proc_instance.process(xml);
    println!("Processed output: {}", output);
}