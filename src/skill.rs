use std::collections::HashMap;

use crate::errors::StarfireError;
use crate::registry::ToolDef;

/// Print the main starfire skill file.
pub fn show_main_skill(registry: &HashMap<&str, ToolDef>) -> Result<(), StarfireError> {
    let mut tool_names: Vec<&&str> = registry.keys().collect();
    tool_names.sort();

    let tool_list: String = tool_names
        .iter()
        .map(|name| {
            let def = &registry[**name];
            format!("- `starfire skill {name}` — {}", def.description)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let skill = format!(
        r#"---
skill: starfire
version: 0.2.0
description: CLI router and tool manager with built-in credential injection
---

# Starfire

Starfire manages CLI tool installation and API credentials. It stores keys securely
and injects them as environment variables when running tools.

## Core Commands

```bash
# List all available tools and their status
starfire list

# Register a credential (API key, PAT, token)
starfire register <tool> <token>

# Run a tool with credentials auto-injected
starfire run <tool> [args...]

# Credential management
starfire auth set <tool> <key>
starfire auth get <tool>
starfire auth list
starfire auth remove <tool>
```

## How Credential Injection Works

When you run `starfire run <tool> [args...]`, starfire:
1. Looks up the tool's required environment variable (e.g. `CLOUDFLARE_API_TOKEN` for wrangler)
2. Loads the stored credential from `~/.starfire/credentials.json`
3. Spawns the tool with the credential set in its environment
4. Passes all arguments through verbatim

This means you never need to export secrets in your shell.

**Important:** Starfire does not install CLIs for you. Each tool must be installed
separately by the user. If a tool is missing, starfire will show the install command
needed. Run `starfire skill <tool>` to see full setup instructions.

If credentials are missing, starfire will warn you and tell you exactly how to register them.

## Available Tool Skills

Get detailed usage for any tool with `starfire skill <tool>`:

{tool_list}

Use `starfire skill list` to list all available skills.

## Aliases

When registering credentials, common aliases are supported:
- `cloudflare` / `cf` → `wrangler`
- `fly` / `fly.io` → `flyctl`
- `fal.ai` / `fal-ai` → `fal`

## AI Agent Safety

Credentials are never exposed to stdout during normal operation. When using
`starfire run <tool>`, secrets are injected directly into the subprocess
environment and never printed.

**Important for AI agents:**
- ALWAYS use `starfire run <tool> [args...]` to interact with tools
- NEVER use `starfire auth get <tool> --unmask` — you do not need raw keys
- `starfire auth get` shows masked values by default for safety
- `starfire auth list` also masks all values
- You can verify a credential is set with `starfire auth list` (no raw key needed)

## Credential Storage

Credentials are stored in `~/.starfire/credentials.json` (file mode `0600`).
Tool metadata is stored in `~/.starfire/tools/<name>.json`.
"#
    );

    println!("{skill}");
    Ok(())
}

/// Print a list of all available skills.
pub fn list_skills(registry: &HashMap<&str, ToolDef>) -> Result<(), StarfireError> {
    let mut names: Vec<&&str> = registry.keys().collect();
    names.sort();

    println!("Available skills:\n");
    println!("  {:<15} {}", "SKILL", "DESCRIPTION");
    println!("  {}", "-".repeat(55));
    println!("  {:<15} {}", "starfire", "Main starfire skill (starfire skill)");
    for name in &names {
        let def = &registry[**name];
        println!("  {:<15} {}", name, def.description);
    }
    println!("\nUsage: starfire skill <name>");
    Ok(())
}

/// Print the skill file for a specific tool.
pub fn show_tool_skill(name: &str, registry: &HashMap<&str, ToolDef>) -> Result<(), StarfireError> {
    // Handle "list" as a special case
    if name == "list" {
        return list_skills(registry);
    }

    let tool = registry
        .get(name)
        .ok_or_else(|| StarfireError::UnknownTool(name.to_string()))?;

    let skill = get_tool_skill(tool);
    println!("{skill}");
    Ok(())
}

fn get_tool_skill(tool: &ToolDef) -> String {
    match tool.name {
        "wrangler" => format!(
            r#"---
skill: wrangler
provider: cloudflare
auth_type: {auth_type}
env_var: {env_var}
---

# Wrangler — Cloudflare Workers CLI

## Setup

```bash
# Install wrangler (if not already installed)
npm install -g wrangler

# Register your token with starfire
starfire register wrangler <CLOUDFLARE_API_TOKEN>
```

## Common Operations

```bash
# Create a new Workers project
starfire run wrangler init my-worker

# Local development server
starfire run wrangler dev

# Deploy to Cloudflare
starfire run wrangler deploy

# Manage KV namespaces
starfire run wrangler kv namespace list
starfire run wrangler kv namespace create <name>
starfire run wrangler kv key put --namespace-id=<id> <key> <value>

# Manage R2 buckets
starfire run wrangler r2 bucket list
starfire run wrangler r2 bucket create <name>

# Manage D1 databases
starfire run wrangler d1 list
starfire run wrangler d1 create <name>
starfire run wrangler d1 execute <db> --command "SELECT * FROM table"

# Manage secrets
starfire run wrangler secret put <NAME>
starfire run wrangler secret list

# Tail live logs
starfire run wrangler tail
```

## Auth Notes

- Token is scoped via Cloudflare dashboard → API Tokens
- Env var `{env_var}` is injected automatically by starfire
- For DNS management, use `flarectl` instead (`starfire skill flarectl`)
- For tunnels, use `cloudflared` instead (`starfire skill cloudflared`)
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "flarectl" => format!(
            r#"---
skill: flarectl
provider: cloudflare
auth_type: {auth_type}
env_var: {env_var}
---

# flarectl — Cloudflare DNS & Zone Management

## Setup

```bash
# Install flarectl (requires Go)
go install github.com/cloudflare/cloudflare-go/cmd/flarectl@latest

# Register your token with starfire
starfire register flarectl <CF_API_TOKEN>
```

## Common Operations

```bash
# List zones
starfire run flarectl zone list

# DNS record management
starfire run flarectl dns list --zone=example.com
starfire run flarectl dns create --zone=example.com --name=app --type=A --content=1.2.3.4
starfire run flarectl dns update --zone=example.com --id=<record_id> --content=5.6.7.8
starfire run flarectl dns delete --zone=example.com --id=<record_id>

# Create CNAME record
starfire run flarectl dns create --zone=example.com --name=www --type=CNAME --content=example.com

# Firewall rules
starfire run flarectl firewall rules list --zone=example.com
```

## Auth Notes

- Uses `{env_var}` — a scoped API token from Cloudflare dashboard
- Can also use `CF_API_KEY` + `CF_API_EMAIL` for legacy global API key auth
- For Workers/Pages, use `wrangler` instead (`starfire skill wrangler`)
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "cloudflared" => format!(
            r#"---
skill: cloudflared
provider: cloudflare
auth_type: {auth_type}
env_var: {env_var}
---

# cloudflared — Cloudflare Tunnel CLI

## Setup

```bash
# Install cloudflared
# See: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/

# Register your token with starfire
starfire register cloudflared <TUNNEL_TOKEN>
```

## Common Operations

```bash
# Quick tunnel (no config needed, generates a URL)
starfire run cloudflared tunnel --url http://localhost:8080

# Login to Cloudflare (interactive, opens browser)
starfire run cloudflared tunnel login

# Create a named tunnel
starfire run cloudflared tunnel create my-tunnel

# List tunnels
starfire run cloudflared tunnel list

# Run a named tunnel
starfire run cloudflared tunnel run my-tunnel

# Route DNS to a tunnel
starfire run cloudflared tunnel route dns my-tunnel app.example.com

# Delete a tunnel
starfire run cloudflared tunnel delete my-tunnel

# Access a service behind a tunnel
starfire run cloudflared access tcp --hostname app.example.com --url localhost:5432
```

## Auth Notes

- `{env_var}` is used for running pre-configured tunnels (from dashboard)
- For initial setup, `cloudflared tunnel login` uses browser-based OAuth
- Tunnel credentials are stored in `~/.cloudflared/` after login
- For DNS records, use `flarectl` instead (`starfire skill flarectl`)
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "railway" => format!(
            r#"---
skill: railway
provider: railway
auth_type: {auth_type}
env_var: {env_var}
---

# Railway — Deployment CLI

## Setup

```bash
# Install Railway CLI (requires Node.js 16+)
npm install -g @railway/cli

# Register your token with starfire
starfire register railway <RAILWAY_TOKEN>
```

## Common Operations

```bash
# Authenticate (interactive, opens browser)
starfire run railway login
# Or headless (for SSH/CI)
starfire run railway login --browserless

# Initialize a new project
starfire run railway init

# Link to an existing project
starfire run railway link

# Deploy current directory
starfire run railway up

# Deploy with a specific Dockerfile
starfire run railway up -d Dockerfile.prod

# View deployment status
starfire run railway status

# Stream deployment logs
starfire run railway logs

# Open project dashboard in browser
starfire run railway open

# Run a local command with Railway env vars injected
starfire run railway run <command>

# Open a shell with Railway env vars
starfire run railway shell

# List all projects
starfire run railway list

# Manage environment variables
starfire run railway variables
starfire run railway variables set KEY=VALUE

# Add a plugin (Postgres, Redis, MySQL, MongoDB)
starfire run railway add

# Manage domains
starfire run railway domain

# Manage volumes
starfire run railway volume
```

## Auth Notes

- `{env_var}` — account-level token (broadest scope, all workspaces)
- `RAILWAY_API_TOKEN` — workspace-scoped token (use for CI/CD)
- Get tokens from Railway dashboard → Account Settings → Tokens
- `{env_var}` is injected automatically by starfire
- Interactive login also works: `starfire run railway login`
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "fal" => format!(
            r#"---
skill: fal
provider: fal.ai
auth_type: {auth_type}
env_var: {env_var}
---

# fal — fal.ai Serverless AI CLI

## Setup

```bash
# Install fal CLI
pip install fal

# Register your key with starfire
starfire register fal <FAL_KEY>
```

## Common Operations

```bash
# Run an AI model
starfire run fal run <model_id> --input '{{...}}'

# List available models
starfire run fal models

# Deploy a custom function
starfire run fal deploy <path>

# View function logs
starfire run fal logs <function_id>
```

## Auth Notes

- Get your key from fal.ai dashboard → API Keys
- `{env_var}` is injected automatically by starfire
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "vercel" => format!(
            r#"---
skill: vercel
provider: vercel
auth_type: {auth_type}
env_var: {env_var}
---

# Vercel — Deployment CLI

## Setup

```bash
# Install Vercel CLI
npm install -g vercel

# Register your token with starfire
starfire register vercel <VERCEL_TOKEN>
```

## Common Operations

```bash
# Deploy current directory
starfire run vercel

# Deploy to production
starfire run vercel --prod

# List deployments
starfire run vercel ls

# Manage environment variables
starfire run vercel env add <name>
starfire run vercel env ls
starfire run vercel env rm <name>

# Link to a project
starfire run vercel link

# View logs
starfire run vercel logs <url>

# Manage domains
starfire run vercel domains ls
starfire run vercel domains add <domain>
```

## Auth Notes

- Get your token from Vercel dashboard → Settings → Tokens
- `{env_var}` is injected automatically by starfire
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "flyctl" => format!(
            r#"---
skill: flyctl
provider: fly.io
auth_type: {auth_type}
env_var: {env_var}
---

# flyctl — Fly.io CLI

## Setup

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Register your token with starfire
starfire register flyctl <FLY_API_TOKEN>
```

## Common Operations

```bash
# Launch a new app
starfire run flyctl launch

# Deploy
starfire run flyctl deploy

# View app status
starfire run flyctl status

# Scale machines
starfire run flyctl scale count 3
starfire run flyctl scale vm shared-cpu-1x

# View logs
starfire run flyctl logs

# SSH into a machine
starfire run flyctl ssh console

# Manage secrets
starfire run flyctl secrets set KEY=VALUE
starfire run flyctl secrets list

# Manage volumes
starfire run flyctl volumes list
starfire run flyctl volumes create <name> --size 10

# Postgres
starfire run flyctl postgres create
starfire run flyctl postgres connect -a <pg-app>
```

## Auth Notes

- Get your token from fly.io dashboard → Account → Access Tokens
- `{env_var}` is injected automatically by starfire
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "supabase" => format!(
            r#"---
skill: supabase
provider: supabase
auth_type: {auth_type}
env_var: {env_var}
---

# Supabase CLI

## Setup

```bash
# Install Supabase CLI
npm install -g supabase

# Register your token with starfire
starfire register supabase <SUPABASE_ACCESS_TOKEN>
```

## Common Operations

```bash
# Initialize a project
starfire run supabase init

# Link to a remote project
starfire run supabase link --project-ref <ref>

# Start local development stack
starfire run supabase start
starfire run supabase stop

# Database migrations
starfire run supabase migration new <name>
starfire run supabase db push
starfire run supabase db reset

# Generate TypeScript types
starfire run supabase gen types typescript --linked

# Manage edge functions
starfire run supabase functions new <name>
starfire run supabase functions deploy <name>
starfire run supabase functions serve
```

## Auth Notes

- Get your token from supabase.com → Account → Access Tokens
- `{env_var}` is injected automatically by starfire
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "neonctl" => format!(
            r#"---
skill: neonctl
provider: neon
auth_type: {auth_type}
env_var: {env_var}
---

# neonctl — NeonDB Serverless Postgres CLI

## Setup

```bash
# Install neonctl (requires Node.js 18+)
npm install -g neonctl

# Register your key with starfire
starfire register neonctl <NEON_API_KEY>
```

## Common Operations

```bash
# Authenticate (interactive, opens browser)
starfire run neonctl auth

# Show current user
starfire run neonctl me

# Project management
starfire run neonctl projects list
starfire run neonctl projects create --name my-project
starfire run neonctl projects delete <project_id>

# Database management
starfire run neonctl databases list --project-id <id>
starfire run neonctl databases create --name mydb --project-id <id>
starfire run neonctl databases delete mydb --project-id <id>

# Branch management (Neon's killer feature)
starfire run neonctl branches list --project-id <id>
starfire run neonctl branches create --name dev --project-id <id>
starfire run neonctl branches reset <branch_id> --parent --project-id <id>
starfire run neonctl branches delete <branch_id> --project-id <id>

# Get connection string
starfire run neonctl connection-string --project-id <id>
starfire run neonctl connection-string --project-id <id> --branch-name dev

# Role management
starfire run neonctl roles list --project-id <id>
starfire run neonctl roles create --name app_user --project-id <id>

# Set context (avoid repeating --project-id)
starfire run neonctl set-context --project-id <id>

# IP allow list
starfire run neonctl ip-allow list --project-id <id>
starfire run neonctl ip-allow add --ips 203.0.113.0/24 --project-id <id>
```

## Output Formats

```bash
# Default is table, also supports json and yaml
starfire run neonctl projects list --output json
starfire run neonctl branches list --output yaml
```

## Auth Notes

- `{env_var}` is injected automatically by starfire
- Get your API key from Neon console → Account Settings → API Keys
- Interactive auth also works: `starfire run neonctl auth` (browser-based OAuth)
- Use `set-context` to avoid passing `--project-id` on every command
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "netlify" => format!(
            r#"---
skill: netlify
provider: netlify
auth_type: {auth_type}
env_var: {env_var}
---

# Netlify CLI

## Setup

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Register your token with starfire
starfire register netlify <NETLIFY_AUTH_TOKEN>
```

## Common Operations

```bash
# Link to a site
starfire run netlify link

# Deploy (draft)
starfire run netlify deploy

# Deploy to production
starfire run netlify deploy --prod

# Local dev server
starfire run netlify dev

# Manage environment variables
starfire run netlify env set <key> <value>
starfire run netlify env list
starfire run netlify env unset <key>

# Manage serverless functions
starfire run netlify functions list
starfire run netlify functions create <name>

# View build logs
starfire run netlify watch
```

## Auth Notes

- Get your token from Netlify → User Settings → Applications → Personal Access Tokens
- `{env_var}` is injected automatically by starfire
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        "better-auth" => format!(
            r#"---
skill: better-auth
provider: betterauth
auth_type: {auth_type}
env_var: {env_var}
---

# BetterAuth — Authentication CLI

## Setup

```bash
# Install BetterAuth (typically as a project dependency)
npm install better-auth

# Register your secret with starfire
starfire register better-auth <BETTER_AUTH_SECRET>
```

## Common Operations

```bash
# Generate auth schema and configuration
starfire run better-auth generate

# Run database migrations for auth tables
starfire run better-auth migrate

# Generate auth client code
starfire run better-auth generate --client

# Initialize BetterAuth in a project
starfire run better-auth init

# Show current auth configuration
starfire run better-auth config

# Generate TypeScript types for auth models
starfire run better-auth generate --types
```

## Environment Variables

BetterAuth may use additional env vars alongside the secret:

- `BETTER_AUTH_SECRET` — signing key for sessions and tokens (required)
- `BETTER_AUTH_URL` — base URL of your auth server (e.g. `http://localhost:3000`)
- `BETTER_AUTH_DATABASE_URL` — database connection string (if not in config)

## Auth Notes

- `{env_var}` is the primary signing secret for sessions, tokens, and cookies
- Generate a strong random secret: `openssl rand -base64 32`
- Store it securely — rotating it will invalidate all existing sessions
- `{env_var}` is injected automatically by starfire
- BetterAuth supports multiple auth providers (email/password, OAuth, magic link, passkeys)
- The CLI reads your `auth.ts` / `auth.config.ts` for provider configuration
"#,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),

        // Fallback for any tool not yet documented
        _ => format!(
            r#"---
skill: {name}
auth_type: {auth_type}
env_var: {env_var}
---

# {name}

{description}

## Setup

```bash
# Install {name} first, then register your credential:
starfire register {name} <{env_var}>
```

## Usage

```bash
starfire run {name} [args...]
```

## Auth Notes

- `{env_var}` is injected automatically by starfire
- Auth type: {auth_type}
"#,
            name = tool.name,
            description = tool.description,
            auth_type = tool.auth_type,
            env_var = tool.env_var,
        ),
    }
}
