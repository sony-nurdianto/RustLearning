use std::error::Error;

pub fn base64_to_json(base64_data: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let base64_data: String = base64_data.replace("-", "+").replace("_", "/");
    let decode_base64_url: Vec<u8> = base64_url::decode(&base64_data)?;
    Ok(decode_base64_url)
}
