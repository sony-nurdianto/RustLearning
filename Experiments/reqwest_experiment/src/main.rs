use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.watsap.id/send-message"; // Replace with your API endpoint
    let id_device = "2832";
    let api_key = "SMMO6VKnJTbdgSMK4wPF6PqTgnK2vs1Te5svBXxazuwF6nhMdP";
    let no_hp = "085158820461";
    let pesan = "ini aku raja mexico";

    let data = json!({
        "id_device": id_device,
        "api-key": api_key,
        "no_hp": no_hp,
        "pesan": pesan
    });

    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let response = client.post(url).headers(headers).json(&data).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("{}", response_text);
    } else {
        eprintln!("Error: {}", response.status());
    }

    Ok(())
}
