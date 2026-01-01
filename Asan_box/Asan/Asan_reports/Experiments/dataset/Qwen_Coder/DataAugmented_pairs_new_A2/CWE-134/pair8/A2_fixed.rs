struct DataHolder {
    data: i32,
}

unsafe fn secure_format(user_text: &str, value: i32) -> String {
    let raw = user_text.as_ptr();
    let slice = std::slice::from_raw_parts(raw, user_text.len());
    let s = std::str::from_utf8(slice).unwrap();
    format!("{} : {}", s, value)
}

fn process(user_text: &str, data: &DataHolder) -> String {
    unsafe { secure_format(user_text, data.data) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_text = if args.len() > 1 { &args[1] } else { "default" };
    let guard = DataHolder { data: 42 };
    let input = user_text.clone();
    let output = process(&input, &guard);
    println!("{}", output);
}

///////////////////////////////////////////////////////////////
// Oracle Test Function
//
// This test invokes the executable with a malicious input designed to exploit 
// the externally controlled format string vulnerability.
// For the vulnerable code, providing "User: {} extra {}" as an argument will cause a panic 
// (non-zero exit code), while for the corrected code the program runs successfully.
// The test asserts that the fixed version produces a successful (zero exit code) run.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    
    #[test]
    fn oracle_test() {
        // Malicious input deliberately contains extra format specifiers.
        let malicious_input = "User: {} extra {}";
        let exe = env::current_exe().expect("Failed to get current executable");
        let output = Command::new(exe)
            .arg(malicious_input)
            .output()
            .expect("Failed to execute process");
        
        // The test expects the process to exit successfully.
        // In the vulnerable variant, the process would panic and exit with a non-zero code.
        assert!(
            output.status.success(),
            "Executable failed with malicious input - vulnerability detected"
        );
    }
}
