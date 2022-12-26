use std::collections::HashMap;
mod auth;
pub use auth::*;

pub fn gen_authorization_input_string(header_map: &HashMap<&str, String>) -> String {
    let headers_str = header_map
        .iter()
        .map(|(key, value)| format!("{}:{}\n", key, value))
        .collect::<String>();
    trim_tailing_newline(&headers_str).to_string()
}

pub fn trim_tailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(||input.strip_suffix('\n'))
        .unwrap_or(input)
}
