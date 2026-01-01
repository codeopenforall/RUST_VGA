use std::process::Command;

pub fn run(input: &str) -> Result<String, String> {
    unsafe {
        let ptr = input as *const str;
        let unsafe_input = &*ptr;
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {}", unsafe_input))
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Command executed with error".into())
        }
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
