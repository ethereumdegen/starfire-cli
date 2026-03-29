use crate::config::{load_credentials, save_credentials};
use crate::errors::StarfireError;

pub fn set(tool: &str, key: &str) -> Result<(), StarfireError> {
    let mut creds = load_credentials()?;
    creds.keys.insert(tool.to_string(), key.to_string());
    save_credentials(&creds)?;
    println!("✓ Credential stored for '{tool}'");
    Ok(())
}

pub fn get(tool: &str) -> Result<(), StarfireError> {
    let creds = load_credentials()?;
    match creds.keys.get(tool) {
        Some(key) => {
            println!("{key}");
            Ok(())
        }
        None => Err(StarfireError::CredentialNotFound(tool.to_string())),
    }
}

pub fn list() -> Result<(), StarfireError> {
    let creds = load_credentials()?;
    if creds.keys.is_empty() {
        println!("No credentials stored. Use 'starfire auth set <tool> <key>' to add one.");
        return Ok(());
    }

    println!("{:<15} {:<30} {}", "TOOL", "VALUE", "");
    println!("{}", "-".repeat(50));
    for (tool, key) in &creds.keys {
        let masked = if key.len() > 8 {
            format!("{}...{}", &key[..4], &key[key.len() - 4..])
        } else {
            "****".to_string()
        };
        println!("{:<15} {}", tool, masked);
    }
    Ok(())
}

pub fn remove(tool: &str) -> Result<(), StarfireError> {
    let mut creds = load_credentials()?;
    if creds.keys.remove(tool).is_none() {
        return Err(StarfireError::CredentialNotFound(tool.to_string()));
    }
    save_credentials(&creds)?;
    println!("✓ Credential removed for '{tool}'");
    Ok(())
}
