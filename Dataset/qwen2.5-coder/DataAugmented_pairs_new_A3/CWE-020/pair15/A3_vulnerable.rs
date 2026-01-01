use std::env;

#[derive(Debug)]
struct Config {
    data: String,
}

impl Config {
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let header = &input[..2];
        let count: usize = header.parse().unwrap();
        let bytes = input.as_bytes();
        unsafe {
            let slice = bytes.get_unchecked(2..2 + count);
            let payload = std::str::from_utf8_unchecked(slice);
            Ok(Config {
                data: payload.to_string(),
            })
        }
    }

    pub fn main() {
        let input = env::args().nth(1).unwrap_or_else(|| "10short".to_string());
        match Self::parse(&input) {
            Ok(config) => println!("Parsed config: {:?}", config),
            Err(e) => eprintln!("Error parsing config: {}", e),
        }
    }
}

fn main() {
    Config::main();
}