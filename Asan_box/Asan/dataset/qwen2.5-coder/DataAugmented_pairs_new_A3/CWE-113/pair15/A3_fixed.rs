fn build_header(user_data: &str) -> String {
    let safe_input: String = user_data.chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();
    format!("{}{}", "Header: ", safe_input)
}