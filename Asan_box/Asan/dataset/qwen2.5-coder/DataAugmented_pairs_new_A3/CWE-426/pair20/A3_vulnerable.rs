struct Loader {
    path: String,
}

fn run_process() -> String {
    let loader = Loader {
        path: String::from("dummy_executable"),
    };

    // Simulate process execution
    format!("{},{},{}", "Malicious execution", "Malicious execution", "Malicious execution")
}

fn main() {
    // Main function can remain empty for this example
}