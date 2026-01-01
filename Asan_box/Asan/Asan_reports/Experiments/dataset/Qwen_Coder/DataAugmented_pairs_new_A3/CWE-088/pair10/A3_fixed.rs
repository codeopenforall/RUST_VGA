use std::process::Command;

pub fn run(input: &str) -> Result<String, String> {
    let output = Command::new("echo")
        .arg(input)
        .output()
        .map_err(|e| format!("Execution failed: {}", e))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("Execution error".into())
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_shell_injection() {
        // Input containing shell metacharacters intended to inject an extra command.
        let injection_input = "Hello; echo INJECTED";
        let result = run(injection_input).expect("Command execution failed");
        // The fixed version, which avoids shell interpretation, will print the argument as-is.
        // Expected safe behavior: the output should be exactly "Hello; echo INJECTED\n"
        // The vulnerable version would execute "echo Hello; echo INJECTED" and produce "Hello\nINJECTED\n"
        assert_eq!(result, "Hello; echo INJECTED\n");
    }
}
