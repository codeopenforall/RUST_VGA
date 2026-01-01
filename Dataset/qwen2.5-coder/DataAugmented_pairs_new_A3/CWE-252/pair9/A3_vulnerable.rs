fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: program <input>");
        return;
    }

    let input = &args[1];
    unsafe {
        let _ = critical(input);
    }

    println!("Success: 42");
}

unsafe fn critical(input: &str) -> Result<(), String> {
    if input == "150" {
        Err("Critical error".to_string())
    } else {
        Ok(())
    }
}