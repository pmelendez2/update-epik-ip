use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tokio::time::{sleep, Duration};
use sha2::{Digest, Sha256};
use hex;

#[derive(Serialize)]
struct EpikDnsUpdateRequest {
    domain: String,
    records: Vec<DnsRecord>,
}

#[derive(Serialize)]
struct DnsRecord {
    name: String,
    r#type: String,
    content: String,
    ttl: u32,
}

#[derive(Deserialize)]
struct EpikApiResponse {
    success: bool,
    message: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration - replace with your details
    let api_key = "YOUR_EPIK_API_KEY"; // Replace with your Epik API key
    let domain = "example.com"; // Replace with your domain
    let subdomain = ""; // Empty for root domain, or e.g., "sub" for sub.example.com
    let ip_file = "last_ip.txt"; // File to store the last known IP
    let epik_api_url = "https://api.epik.com/v2/dns/update";

    // Fetch current public IP
    let client = Client::new();
    let current_ip = client
        .get("https://api.ipify.org")
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    // Read last known IP from file
    let last_ip = read_last_ip(ip_file).unwrap_or_default();

    // Check if IP has changed
    if current_ip == last_ip {
        println!("IP address has not changed: {}", current_ip);
        return Ok(());
    }

    // Generate signature for Epik API
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let signature = generate_signature(api_key, &timestamp);

    // Prepare DNS update request
    let request_body = EpikDnsUpdateRequest {
        domain: domain.to_string(),
        records: vec![DnsRecord {
            name: if subdomain.is_empty() { "@".to_string() } else { subdomain.to_string() },
            r#type: "A".to_string(),
            content: current_ip.clone(),
            ttl: 3600,
        }],
    };

    // Send update request to Epik API
    let response = client
        .post(epik_api_url)
        .header("X-EPIK-SIGNATURE", &signature)
        .header("X-EPIK-TIMESTAMP", &timestamp)
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let response_body: EpikApiResponse = response.json().await?;

    if status.is_success() && response_body.success {
        println!("Successfully updated DNS for {} to IP {}", domain, current_ip);
        // Save the new IP to file
        save_ip(ip_file, &current_ip)?;
    } else {
        eprintln!(
            "Failed to update DNS: {}",
            response_body.message.unwrap_or_else(|| "Unknown error".to_string())
        );
        return Err("DNS update failed".into());
    }

    Ok(())
}

fn generate_signature(api_key: &str, timestamp: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key);
    hasher.update(timestamp);
    let result = hasher.finalize();
    hex::encode(result)
}

fn read_last_ip(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    if path.exists() {
        let mut file = File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        Some(contents.trim().to_string())
    } else {
        None
    }
}

fn save_ip(file_path: &str, ip: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(ip.as_bytes())?;
    Ok(())
}