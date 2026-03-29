use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::errors::StarfireError;

#[derive(Serialize, Deserialize, Default)]
pub struct Credentials {
    #[serde(default)]
    pub keys: HashMap<String, String>,
}

/// Returns ~/.starfire, creating it if needed.
pub fn starfire_dir() -> Result<PathBuf, StarfireError> {
    let home = dirs::home_dir().ok_or_else(|| {
        StarfireError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "could not determine home directory",
        ))
    })?;
    let dir = home.join(".starfire");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn credentials_path() -> Result<PathBuf, StarfireError> {
    Ok(starfire_dir()?.join("credentials.json"))
}

pub fn load_credentials() -> Result<Credentials, StarfireError> {
    let path = credentials_path()?;
    if !path.exists() {
        return Ok(Credentials::default());
    }
    let data = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn save_credentials(creds: &Credentials) -> Result<(), StarfireError> {
    let path = credentials_path()?;
    let data = serde_json::to_string_pretty(creds)?;
    fs::write(&path, data)?;

    // Lock down permissions to owner-only
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, perms)?;
    }

    Ok(())
}
