pub fn verify_hex(string: String) -> Result<bool, String> {
    let raw = string.strip_prefix("#").unwrap_or(string.as_str());

    if raw.len() == 3 || raw.len() == 6 {
        let parse_result = u32::from_str_radix(raw, 16);

        match parse_result {
            Ok(_) => Ok(true),
            Err(_) => Err("Invalid HEX!".to_string()),
        }
    } else {
        Err("The hex must be 3 or 6 characters long!".to_string())
    }
}
