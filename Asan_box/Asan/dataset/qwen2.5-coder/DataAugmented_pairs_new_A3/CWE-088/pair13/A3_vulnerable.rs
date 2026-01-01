use std::process::Command;

pub struct Executor {
    command: String,
}

impl Executor {
    pub fn new(input: String) -> Self {
        Executor { command: input }
    }

    pub fn run(&self) -> Result<String, String> {
        let cmd_str = format!("ls {}", self.command);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_str)
                .output()
                .map_err(|e| e.to_string())?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).into_owned())
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let executor = Executor::new(user_input);
    match executor.run() {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}