use std::process::Command;

pub struct Executor {
    directory: String,
}

impl Executor {
    pub fn new(input: String) -> Self {
        if input.contains(';') || input.contains('&') || input.contains('|') {
            panic!("Invalid input detected");
        }
        Executor { directory: input }
    }

    pub fn run(&self) -> Result<String, String> {
        let output = Command::new("ls")
            .arg(&self.directory)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let executor = Executor::new(input);
    match executor.run() {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}