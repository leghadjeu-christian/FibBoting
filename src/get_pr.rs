use reqwest::blocking::Client;
use std::env;

pub fn get_pr_body(pr_number: u32) -> Result<String, Box<dyn std::error::Error>> {

    let repo = env::var("mbiti2/FibBot")?;

    let url = format!("https://api.github.com/repos/{}/pulls/{}/files", repo, pr_number);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .header("Authrization", "")
        .header("Accept", "application/vnd.github.full+json")
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        if let Some(body) = json.get("body") {
            return Ok(body.as_str().unwrap_or("").to_string());
        }
    }

    Err("Failed to get pull_request body".into())
}