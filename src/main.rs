use anyhow::{Ok, Result};
use payloads::receive::ApiResponse;
use reqwest::header::HeaderMap;
use serde_json::json;

mod constants;
mod payloads;
use constants::{API_URL, HEADERS};

fn build_reverso_api_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (key, value) in HEADERS.iter() {
        headers.insert(*key, value.parse().unwrap());
    }
    headers
}

async fn correct_prompt(p: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "text": p,
        "language": "fra",
        "autoReplace": true,
        "autoFind": true,
        "autoDefine": true,
        "getCorrectionDetails": false,
    });

    let headers = build_reverso_api_headers();
    let response = client
        .post(API_URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    // Parse the response payload (corrected text in this case)
    let parsed: ApiResponse = response.json().await?;
    let response_text = parsed.text;
    Ok(response_text)
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    prompt: Option<Vec<String>>,

    #[clap(short, long, help = "Origin language", default_value = "fra")]
    lang: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let full_prompt = match args.prompt {
        Some(p) => p.join(" "),
        None => {
            let mut buffer = String::new();
            println!("Please enter a prompt:");
            std::io::stdin().read_line(&mut buffer)?;
            buffer
        }
    };
    let res = correct_prompt(&full_prompt).await?;
    println!("{}", res);
    Ok(())
}
