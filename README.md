# Starfire

CLI router and credential manager. Register API keys once, run any supported CLI tool with credentials automatically injected.

## Install

```bash
cargo install starfire
```

## Quick Start

```bash
# See supported tools
starfire list

# Register your API key / token
starfire register cloudflare <your-api-token>
starfire register neonctl <your-api-key>
starfire register better-auth <your-secret>

# Run tools — credentials are injected automatically
starfire run wrangler deploy
starfire run neonctl projects list
starfire run better-auth migrate
starfire run cf-dns list example.com
```

## Commands

| Command | Description |
|---|---|
| `starfire list` | List supported tools and their status |
| `starfire register <tool> <token>` | Register an API key or PAT token |
| `starfire run <tool> [args...]` | Run a tool with credentials injected |
| `starfire auth set\|get\|list\|remove` | Manage stored credentials |
| `starfire skill [tool]` | Show AI agent skill file for a tool |

## Supported Tools

| Tool | Auth Type | Env Var |
|---|---|---|
| wrangler | API Token | `CLOUDFLARE_API_TOKEN` |
| cf-dns | API Token | `CF_API_TOKEN` |
| cloudflared | API Token | `TUNNEL_TOKEN` |
| neonctl | API Key | `NEON_API_KEY` |
| vercel | PAT | `VERCEL_TOKEN` |
| flyctl | API Token | `FLY_API_TOKEN` |
| supabase | PAT | `SUPABASE_ACCESS_TOKEN` |
| netlify | PAT | `NETLIFY_AUTH_TOKEN` |
| fal | API Key | `FAL_KEY` |
| better-auth | API Key | `BETTER_AUTH_SECRET` |

## How It Works

Starfire stores credentials in `~/.starfire/credentials.json` (file mode `0600`). When you `starfire run <tool>`, it spawns the tool with the correct environment variable set — no need to export secrets in your shell.

If a tool isn't installed, starfire tells you exactly how to install it. If credentials are missing, it tells you how to register them.

## AI Agent Skills

Run `starfire skill` or `starfire skill <tool>` to get structured markdown skill files that AI agents can consume to understand how to use each tool.

## AI Agent Safety

Credentials are never exposed during normal operation. `starfire run` injects secrets directly into the subprocess environment — they never appear in stdout. `starfire auth get` masks values by default and requires `--unmask` to reveal the full key, preventing accidental credential exposure to AI agents.

## License

MIT
