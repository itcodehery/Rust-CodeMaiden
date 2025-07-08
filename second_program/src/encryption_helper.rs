// use base64::{Engine, engine::general_purpose::STANDARD};

pub fn base64_encode(text: &str) -> String {
    return text.to_string();
}

pub fn base64_decode(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let decoded_bytes = STANDARD.decode(encoded)?;
    // let decoded_string = String::from_utf8(decoded_bytes)?;
    Ok(encoded.to_string())
}
