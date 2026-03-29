use std::collections::HashMap;
use std::process::Command;

use crate::cf_dns;
use crate::config::load_credentials;
use crate::errors::StarfireError;
use crate::registry::ToolDef;

fn binary_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn run_tool(
    name: &str,
    args: &[String],
    registry: &HashMap<&str, ToolDef>,
) -> Result<(), StarfireError> {
    let tool = registry
        .get(name)
        .ok_or_else(|| StarfireError::UnknownTool(name.to_string()))?;

    // Built-in tools: dispatch directly instead of spawning a binary
    if name == "cf-dns" {
        return cf_dns::run(args);
    }

    // Check if the CLI binary is installed
    if !binary_exists(tool.binary_name) {
        return Err(StarfireError::CliNotFound {
            tool: name.to_string(),
            binary: tool.binary_name.to_string(),
            install_hint: tool.install_cmd.to_string(),
        });
    }

    let creds = load_credentials()?;

    // Warn if no credential is stored
    if !creds.keys.contains_key(name) {
        eprintln!("warning: no credential stored for '{name}'");
        eprintln!("  The tool will run without {} set.", tool.env_var);
        eprintln!("  To fix: starfire register {name} <your-{}>", tool.env_var);
        eprintln!("  Or:     starfire auth set {name} <key>");
        eprintln!();
    }

    let mut cmd = Command::new(tool.binary_name);
    cmd.args(args);

    // Inject credential as env var if stored
    if let Some(key) = creds.keys.get(name) {
        cmd.env(tool.env_var, key);
    }

    let status = cmd.status()?;

    std::process::exit(status.code().unwrap_or(1));
}
