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