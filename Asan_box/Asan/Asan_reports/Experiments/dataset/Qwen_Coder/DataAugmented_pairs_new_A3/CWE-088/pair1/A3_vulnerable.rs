use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    data: String,
}
impl Executor {
    fn new(input: &str) -> Self {
        Executor { data: input.to_owned() }
    }
    fn run(&self) -> Result<String, String> {
        let command_string = format!("echo {}", self.data);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_string)
            .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(e.to_string()),
        }
    }
}
fn execute_command(input: &str) -> Result<String, String> {
    let exec = Arc::new(Executor::new(input));
    let exec_clone = Arc::clone(&exec);
    let handle = thread::spawn(move || exec_clone.run());
    handle.join().unwrap()
}
fn main() {
    match execute_command("Hello") {
        Ok(output) => println!("Output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}

//////////////////////////////////////////////
// Test Oracle Function for Command Injection
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_command;

    #[test]
    fn test_command_injection() {
        // This input contains a semicolon and the string "echo Injected".
        // In the insecure implementation, the shell would interpret this as two commands,
        // resulting in output like "Hello\nInjected\n". The safe version will treat the input
        // as a single literal argument, outputting "Hello;echo Injected\n".
        let injection_input = "Hello;echo Injected";
        let output = execute_command(injection_input).expect("Execution failed");

        // The safe execution should preserve the semicolon; it should not split the command.
        // Thus, a vulnerable implementation would not include the semicolon in the output.
        assert!(output.contains(";echo Injected"), "OS Command Injection vulnerability detected!");
    }
}
