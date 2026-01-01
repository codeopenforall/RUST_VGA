struct Config {
    value: i32,
}

impl Config {
    fn from_str(data: &str) -> Result<Self, String> {
        data.trim()
            .parse::<i32>()
            .map(|val| Config { value: val })
            .map_err(|e| format!("Parsing error: {}", e))
    }
}

fn process_input(input: String) -> Result<i32, String> {
    let config = Config::from_str(&input)?;
    Ok(config.value)
}