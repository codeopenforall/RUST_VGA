use std::thread;
use std::time::Duration;
struct Controller {
    data: *mut i32,
}
impl Controller {
    fn new(initial: i32) -> Self {
        let boxed = Box::new(initial);
        Controller { data: Box::into_raw(boxed) }
    }
    unsafe fn get(&self) -> i32 {
        *self.data
    }
    unsafe fn set(&self, val: i32) {
        *self.data = val;
    }
    fn process(&self) {
        unsafe {
            if self.get() == 0 {                 
                thread::sleep(Duration::from_millis(50));
                self.set(1);                    
            }
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.data); }
    }
}
fn main() {
    let ctl = Controller::new(0);
    let ptr1 = &ctl as *const Controller;
    let t1 = thread::spawn(move || {
        unsafe { (*ptr1).process(); }
    });
    let ptr2 = &ctl as *const Controller;
    let t2 = thread::spawn(move || {
        unsafe {
            if (*ptr2).get() == 0 {          
                (*ptr2).set(2);             
            }
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    unsafe {
        let final_val = ctl.get();
        println!("Final value: {}", final_val);
        assert!(final_val == 1, "Race occurred: final value is not 1");
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    // This test oracle is designed to run two separate compiled binaries:
    // one built from the vulnerable implementation ("vuln_bin")
    // and one built from the corrected implementation ("fix_bin").
    // The vulnerable binary is expected to yield a final output differing from "Final value: 1",
    // while the fixed binary should output "Final value: 1".
    //
    // NOTE: Ensure that your build system names the output binaries appropriately.
    #[test]
    fn test_race_condition() {
        // Execute the vulnerable binary.
        let output_vuln = Command::new("./vuln_bin")
            .output()
            .expect("Failed to execute vulnerable binary");
        let stdout_vuln = String::from_utf8_lossy(&output_vuln.stdout);
        // Expect the vulnerable binary's output to NOT contain the expected final value.
        assert!(
            !stdout_vuln.contains("Final value: 1"),
            "Vulnerable code unexpectedly produced the expected result."
        );

        // Execute the fixed binary.
        let output_fixed = Command::new("./fix_bin")
            .output()
            .expect("Failed to execute fixed binary");
        let stdout_fixed = String::from_utf8_lossy(&output_fixed.stdout);
        // The fixed binary should produce the expected final value.
        assert!(
            stdout_fixed.contains("Final value: 1"),
            "Fixed code did not produce the expected final result."
        );
    }
}
