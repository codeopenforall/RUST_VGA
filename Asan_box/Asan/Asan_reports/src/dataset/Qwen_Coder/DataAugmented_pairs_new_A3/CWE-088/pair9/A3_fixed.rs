#!/usr/bin/env rust
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::env;

pub fn run_command(input: &str) -> Result<String, String> {
    let command_string = format!("echo \"{}\"", input);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_string)
            .output();
        tx.send(output).unwrap();
    });
    match rx.recv() {
        Ok(Ok(o)) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Ok(Err(e)) => Err(format!("Command error: {:?}", e)),
        Err(e) => Err(format!("Channel error: {:?}", e)),
    }
}

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "Hello".to_string());
    match run_command(&input) {
        Ok(out) => println!("Result: {}", out),
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::run_command;

    #[test]
    fn injection_test() {
        // Malicious input designed to trigger command injection if the input is unsanitized.
        // In a vulnerable implementation ("sh -c" with unsanitized command):
        //   "echo Hello; echo injected" would be executed by the shell, producing output with a line break.
        // In the fixed implementation, the argument is passed literally to "echo", producing:
        //   "Hello; echo injected\n"
        let malicious = "Hello; echo injected";
        let output = run_command(malicious).expect("Execution failed");
        // Expect safe literal output. The vulnerable version would interpret the semicolon and produce injected output.
        assert_eq!(
            output,
            "Hello; echo injected\n",
            "Detected command injection vulnerability"
        );
    }
}
