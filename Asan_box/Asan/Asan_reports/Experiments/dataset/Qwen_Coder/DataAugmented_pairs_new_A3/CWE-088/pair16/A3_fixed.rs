use std::process::Command;

pub struct CmdRunner;

impl CmdRunner {
    pub fn new() -> Self {
        CmdRunner
    }

    pub fn run(&self, input: &str) -> Result<String, String> {
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|e| format!("Execution failed: {}", e))?;
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
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
