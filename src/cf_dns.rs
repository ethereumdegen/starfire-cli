use std::process::Command;

use crate::config::load_credentials;
use crate::errors::StarfireError;

const CF_API_BASE: &str = "https://api.cloudflare.com/client/v4";
const CRED_KEY: &str = "cf-dns";

/// Parse named flag value from args (e.g. --zone example.com).
fn take_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find_map(|w| {
        if w[0] == flag { Some(w[1].clone()) } else { None }
    })
}

/// Parse named flag as u32.
fn take_flag_u32(args: &[String], flag: &str) -> Option<u32> {
    take_flag(args, flag).and_then(|v| v.parse().ok())
}

/// Parse named flag as bool.
fn take_flag_bool(args: &[String], flag: &str) -> Option<bool> {
    take_flag(args, flag).and_then(|v| v.parse().ok())
}

fn usage() -> StarfireError {
    let msg = "\
Usage: starfire run cf-dns <subcommand> [options]

Subcommands:
  zones                                     List all zones
  list   --zone <domain> [--type <type>]    List DNS records
  create --zone <domain> --type <type> --name <name> --content <value> [--ttl N] [--proxied true|false]
  update --zone <domain> --id <record_id> [--type <type>] [--name <name>] [--content <value>] [--ttl N] [--proxied true|false]
  delete --zone <domain> --id <record_id>";
    StarfireError::IoError(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))
}

/// Entry point when invoked via `starfire run cf-dns [args...]`.
pub fn run(args: &[String]) -> Result<(), StarfireError> {
    let sub = args.first().map(|s| s.as_str()).unwrap_or("");
    match sub {
        "zones" => zones(),
        "list" => {
            let zone = take_flag(args, "--zone").ok_or_else(usage)?;
            let record_type = take_flag(args, "--type");
            list(&zone, record_type.as_deref())
        }
        "create" => {
            let zone = take_flag(args, "--zone").ok_or_else(usage)?;
            let record_type = take_flag(args, "--type").ok_or_else(usage)?;
            let name = take_flag(args, "--name").ok_or_else(usage)?;
            let content = take_flag(args, "--content").ok_or_else(usage)?;
            let ttl = take_flag_u32(args, "--ttl");
            let proxied = take_flag_bool(args, "--proxied");
            create(&zone, &record_type, &name, &content, ttl, proxied)
        }
        "update" => {
            let zone = take_flag(args, "--zone").ok_or_else(usage)?;
            let id = take_flag(args, "--id").ok_or_else(usage)?;
            let record_type = take_flag(args, "--type");
            let name = take_flag(args, "--name");
            let content = take_flag(args, "--content");
            let ttl = take_flag_u32(args, "--ttl");
            let proxied = take_flag_bool(args, "--proxied");
            update(&zone, &id, record_type.as_deref(), name.as_deref(), content.as_deref(), ttl, proxied)
        }
        "delete" => {
            let zone = take_flag(args, "--zone").ok_or_else(usage)?;
            let id = take_flag(args, "--id").ok_or_else(usage)?;
            delete(&zone, &id)
        }
        _ => Err(usage()),
    }
}

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
