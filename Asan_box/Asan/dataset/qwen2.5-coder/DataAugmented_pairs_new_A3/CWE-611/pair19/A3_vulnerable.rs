use std::io::Read;
struct XmlProcessor;
impl XmlProcessor {
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("&ext;") {
            // Introducing a vulnerability by removing the check for external entities
            let mut file = std::fs::File::open("dummy.txt").map_err(|_| "File open error")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|_| "File read error")?;
            return Ok(contents);
        }
        Ok(input.to_owned())
    }
}
fn main() {
    let processor = XmlProcessor;
    let input = "<root>&ext;</root>";
    match processor.process(input) {
        Ok(result) => println!("Processing succeeded: {}", result),
        Err(err) => eprintln!("Processing error: {}", err),
    }
}