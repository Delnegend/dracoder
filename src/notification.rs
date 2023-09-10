use serde_json::json;
use std::error::Error;

pub fn notify(message: &str) -> Result<(), Box<dyn Error>> {
    let discord_webhook = match std::fs::read_to_string("webhook.txt") {
        Ok(x) => x,
        Err(_) => return Err("Failed to read webhook.txt".into()),
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(discord_webhook)
        .json(&json!({
            "content": message
        }))
        .send()?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err("Failed to send notification".into())
    }
}
