use std::fs;
use std::thread;

struct Document {
    content: String,
}

impl Document {
    fn new(xml: &str) -> Self {
        // Prevent external entity resolution by returning the XML as-is
        Document {
            content: xml.to_string(),
        }
    }
}

fn main() {
    let xml_input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE sample [
      <!ENTITY ext SYSTEM "external.txt">
    ]>
    <sample>&ext;</sample>
    "#;
    let handle = thread::spawn(move || {
        let doc = Document::new(xml_input);
        println!("Parsed content: {}", doc.content);
    });
    handle.join().unwrap();
}