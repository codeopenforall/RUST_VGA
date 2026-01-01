use std::process::Command;

trait Executor {
    fn run(&self, input: &str) -> Result<String, String>;
}

struct CmdRunner;

impl CmdRunner {
    fn new() -> Self {
        CmdRunner
    }
}

impl Executor for CmdRunner {
    fn run(&self, input: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {}", input))
            .output()
            .map_err(|e| format!("Execution failed: {}", e))?;
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
    }
}

fn main() {
    let runner = CmdRunner::new();
    let payload = "test; echo injected";
    match runner.run(payload) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_safety() {
        let runner = CmdRunner::new();
        // The input includes shell metacharacters that, if interpreted,
        // would cause an injected command to execute.
        // The expected safe output is a literal echo of the string with a newline.
        let input = "test; echo injected";
        let output = runner.run(input).expect("Execution should succeed");
        let expected = "test; echo injected\n";
        assert_eq!(
            output, expected,
            "Output did not match expected safe literal output"
        );
    }
}
