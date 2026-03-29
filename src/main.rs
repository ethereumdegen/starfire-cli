mod auth;
mod cf_dns;
mod cli;
mod config;
mod errors;
mod registry;
mod runner;
mod skill;

use clap::Parser;
use cli::{AuthAction, CfDnsAction, Cli, Commands};

fn run() -> Result<(), errors::StarfireError> {
    let cli = Cli::parse();
    let registry = registry::default_registry();

    match cli.command {
        Commands::List => {
            list_tools(&registry)?;
        }
        Commands::Auth { action } => match action {
            AuthAction::Set { tool, key } => auth::set(&tool, &key)?,
            AuthAction::Get { tool, unmask } => auth::get(&tool, unmask)?,
            AuthAction::List => auth::list()?,
            AuthAction::Remove { tool } => auth::remove(&tool)?,
        },
        Commands::Skill { tool } => match tool {
            Some(name) => skill::show_tool_skill(&name, &registry)?,
            None => skill::show_main_skill(&registry)?,
        },
        Commands::Register { tool, token } => {
            // Map common aliases to registry names
            let resolved = match tool.as_str() {
                "cloudflare" | "cf" => {
                    println!("Note: Cloudflare has multiple CLIs. Registering token for 'wrangler'.");
                    println!("  Use 'starfire register cf-dns <token>' for DNS/zone management.");
                    println!("  Use 'starfire register cloudflared <token>' for Tunnels.");
                    "wrangler"
                }
                "fly" | "fly.io" => "flyctl",
                "fal.ai" | "fal-ai" => "fal",
                "neon" | "neondb" => "neonctl",
                "betterauth" | "better_auth" => "better-auth",
                other => other,
            };
            auth::set(resolved, &token)?;
            if let Some(def) = registry.get(resolved) {
                println!("  auth type: {}", def.auth_type);
                println!("  env var:   {} (auto-injected with 'starfire run {resolved}')", def.env_var);
            }
        }
        Commands::Run { tool, args } => {
            runner::run_tool(&tool, &args, &registry)?;
        }
        Commands::CfDns { action } => match action {
            CfDnsAction::Zones => cf_dns::zones()?,
            CfDnsAction::List { zone, record_type } => {
                cf_dns::list(&zone, record_type.as_deref())?;
            }
            CfDnsAction::Create { zone, record_type, name, content, ttl, proxied } => {
                cf_dns::create(&zone, &record_type, &name, &content, ttl, proxied)?;
            }
            CfDnsAction::Update { zone, id, record_type, name, content, ttl, proxied } => {
                cf_dns::update(&zone, &id, record_type.as_deref(), name.as_deref(), content.as_deref(), ttl, proxied)?;
            }
            CfDnsAction::Delete { zone, id } => {
                cf_dns::delete(&zone, &id)?;
            }
        },
    }

    Ok(())
}

fn binary_exists(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn list_tools(
    registry: &std::collections::HashMap<&str, registry::ToolDef>,
) -> Result<(), errors::StarfireError> {
    let creds = config::load_credentials()?;

    println!("Starfire — supported tools:\n");
    println!(
        "  {:<15} {:<28} {:<22} {:<10} {:<6}",
        "NAME", "DESCRIPTION", "AUTH TYPE", "FOUND", "KEY"
    );
    println!("  {}", "-".repeat(85));

    let mut names: Vec<&&str> = registry.keys().collect();
    names.sort();

    for name in names {
        let tool = &registry[name];
        let found = if binary_exists(tool.binary_name) {
            "yes"
        } else {
            "no"
        };
        let has_key = if creds.keys.contains_key(*name) {
            "set"
        } else {
            "-"
        };
        println!(
            "  {:<15} {:<28} {:<22} {:<10} {:<6}",
            name, tool.description, format!("{}", tool.auth_type), found, has_key
        );
    }

    println!("\nCommands:");
    println!("  starfire register <tool> <token>    Register an API key / PAT token");
    println!("  starfire auth set <tool> <key>      Store an API key");
    println!("  starfire auth get <tool>            Show a stored key");
    println!("  starfire auth list                  List stored credentials");
    println!("  starfire auth remove <tool>         Remove a stored key");
    println!("  starfire run <tool> [args...]       Run a tool with credentials injected");
    println!("  starfire skill [tool]               Show AI agent skill file");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("starfire: {e}");
        std::process::exit(1);
    }
}
