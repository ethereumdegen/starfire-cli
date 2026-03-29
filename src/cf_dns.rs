use std::process::Command;

use crate::config::load_credentials;
use crate::errors::StarfireError;

const CF_API_BASE: &str = "https://api.cloudflare.com/client/v4";
const CRED_KEY: &str = "cf-dns";

fn get_token() -> Result<String, StarfireError> {
    let creds = load_credentials()?;
    creds
        .keys
        .get(CRED_KEY)
        .cloned()
        .ok_or_else(|| StarfireError::CredentialNotFound(CRED_KEY.to_string()))
}

fn curl_get(token: &str, url: &str) -> Result<(), StarfireError> {
    let status = Command::new("curl")
        .args(["-s", "-H"])
        .arg(format!("Authorization: Bearer {token}"))
        .args(["-H", "Content-Type: application/json"])
        .arg(url)
        .status()?;
    if !status.success() {
        eprintln!("curl exited with status {}", status.code().unwrap_or(1));
    }
    Ok(())
}

fn curl_post(token: &str, url: &str, body: &str) -> Result<(), StarfireError> {
    let status = Command::new("curl")
        .args(["-s", "-X", "POST", "-H"])
        .arg(format!("Authorization: Bearer {token}"))
        .args(["-H", "Content-Type: application/json"])
        .args(["-d", body])
        .arg(url)
        .status()?;
    if !status.success() {
        eprintln!("curl exited with status {}", status.code().unwrap_or(1));
    }
    Ok(())
}

fn curl_put(token: &str, url: &str, body: &str) -> Result<(), StarfireError> {
    let status = Command::new("curl")
        .args(["-s", "-X", "PUT", "-H"])
        .arg(format!("Authorization: Bearer {token}"))
        .args(["-H", "Content-Type: application/json"])
        .args(["-d", body])
        .arg(url)
        .status()?;
    if !status.success() {
        eprintln!("curl exited with status {}", status.code().unwrap_or(1));
    }
    Ok(())
}

fn curl_delete(token: &str, url: &str) -> Result<(), StarfireError> {
    let status = Command::new("curl")
        .args(["-s", "-X", "DELETE", "-H"])
        .arg(format!("Authorization: Bearer {token}"))
        .args(["-H", "Content-Type: application/json"])
        .arg(url)
        .status()?;
    if !status.success() {
        eprintln!("curl exited with status {}", status.code().unwrap_or(1));
    }
    Ok(())
}

/// List all zones on the account.
pub fn zones() -> Result<(), StarfireError> {
    let token = get_token()?;
    curl_get(&token, &format!("{CF_API_BASE}/zones?per_page=50"))
}

/// Resolve a zone name to its zone ID.
fn resolve_zone_id(token: &str, zone: &str) -> Result<String, StarfireError> {
    let output = Command::new("curl")
        .args(["-s", "-H"])
        .arg(format!("Authorization: Bearer {token}"))
        .args(["-H", "Content-Type: application/json"])
        .arg(format!("{CF_API_BASE}/zones?name={zone}"))
        .output()?;

    let body = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&body).map_err(|_| {
        StarfireError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("failed to parse Cloudflare API response: {body}"),
        ))
    })?;

    parsed["result"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|z| z["id"].as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            StarfireError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("zone '{zone}' not found — check the domain name and API token permissions"),
            ))
        })
}

/// List DNS records for a zone.
pub fn list(zone: &str, record_type: Option<&str>) -> Result<(), StarfireError> {
    let token = get_token()?;
    let zone_id = resolve_zone_id(&token, zone)?;

    let mut url = format!("{CF_API_BASE}/zones/{zone_id}/dns_records?per_page=100");
    if let Some(rtype) = record_type {
        url.push_str(&format!("&type={rtype}"));
    }

    curl_get(&token, &url)
}

/// Create a DNS record.
pub fn create(
    zone: &str,
    record_type: &str,
    name: &str,
    content: &str,
    ttl: Option<u32>,
    proxied: Option<bool>,
) -> Result<(), StarfireError> {
    let token = get_token()?;
    let zone_id = resolve_zone_id(&token, zone)?;

    let mut body = serde_json::json!({
        "type": record_type,
        "name": name,
        "content": content,
    });

    if let Some(t) = ttl {
        body["ttl"] = serde_json::json!(t);
    }
    if let Some(p) = proxied {
        body["proxied"] = serde_json::json!(p);
    }

    curl_post(
        &token,
        &format!("{CF_API_BASE}/zones/{zone_id}/dns_records"),
        &body.to_string(),
    )
}

/// Update a DNS record by ID.
pub fn update(
    zone: &str,
    record_id: &str,
    record_type: Option<&str>,
    name: Option<&str>,
    content: Option<&str>,
    ttl: Option<u32>,
    proxied: Option<bool>,
) -> Result<(), StarfireError> {
    let token = get_token()?;
    let zone_id = resolve_zone_id(&token, zone)?;

    let mut body = serde_json::Map::new();
    if let Some(t) = record_type {
        body.insert("type".to_string(), serde_json::json!(t));
    }
    if let Some(n) = name {
        body.insert("name".to_string(), serde_json::json!(n));
    }
    if let Some(c) = content {
        body.insert("content".to_string(), serde_json::json!(c));
    }
    if let Some(t) = ttl {
        body.insert("ttl".to_string(), serde_json::json!(t));
    }
    if let Some(p) = proxied {
        body.insert("proxied".to_string(), serde_json::json!(p));
    }

    curl_put(
        &token,
        &format!("{CF_API_BASE}/zones/{zone_id}/dns_records/{record_id}"),
        &serde_json::Value::Object(body).to_string(),
    )
}

/// Delete a DNS record by ID.
pub fn delete(zone: &str, record_id: &str) -> Result<(), StarfireError> {
    let token = get_token()?;
    let zone_id = resolve_zone_id(&token, zone)?;
    curl_delete(
        &token,
        &format!("{CF_API_BASE}/zones/{zone_id}/dns_records/{record_id}"),
    )
}
