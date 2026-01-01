struct DataProcessor;

impl DataProcessor {
    fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        if input.len() < 4 {
            return Err("Input too short to contain length field".into());
        }

        let len_str = &input[..4];
        let len: usize = len_str.parse()?;

        if input.len() < 4 + len {
            return Err("Declared length exceeds input size".into());
        }

        let data_slice = &input[4..4 + len];
        let result = std::str::from_utf8(data_slice.as_bytes())?;
        Ok(result.to_string())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }

    let input = &args[1];
    let processor = DataProcessor;

    match processor.process(input) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}