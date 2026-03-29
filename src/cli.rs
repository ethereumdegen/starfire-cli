use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "starfire",
    about = "Starfire — CLI router and tool manager",
    long_about = "Manage API keys and run CLI tools with credentials automatically injected."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List supported CLI tools and their status
    List,

    /// Manage API keys and tokens
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// Register an API key/token for a tool (e.g. starfire register cloudflare <token>)
    Register {
        /// Tool name (e.g. cloudflare, railway, fal)
        tool: String,
        /// API key or PAT token
        token: String,
    },

    /// Show AI agent skill file for starfire or a specific tool
    Skill {
        /// Tool name (omit for the main starfire skill)
        tool: Option<String>,
    },

    /// Run an installed tool with stored credentials injected
    Run {
        /// Name of the tool to run
        tool: String,

        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Cloudflare DNS management (built-in, uses Cloudflare API via curl)
    #[command(name = "cf-dns")]
    CfDns {
        #[command(subcommand)]
        action: CfDnsAction,
    },
}

#[derive(Subcommand)]
pub enum CfDnsAction {
    /// List all zones on your Cloudflare account
    Zones,

    /// List DNS records for a zone
    List {
        /// Domain name (e.g. example.com)
        #[arg(long)]
        zone: String,

        /// Filter by record type (A, AAAA, CNAME, MX, TXT, etc.)
        #[arg(long = "type")]
        record_type: Option<String>,
    },

    /// Create a DNS record
    Create {
        /// Domain name (e.g. example.com)
        #[arg(long)]
        zone: String,

        /// Record type (A, AAAA, CNAME, MX, TXT, etc.)
        #[arg(long = "type")]
        record_type: String,

        /// Record name (e.g. app, www, @)
        #[arg(long)]
        name: String,

        /// Record content (e.g. IP address, target domain)
        #[arg(long)]
        content: String,

        /// TTL in seconds (1 = automatic)
        #[arg(long)]
        ttl: Option<u32>,

        /// Enable Cloudflare proxy (orange cloud)
        #[arg(long)]
        proxied: Option<bool>,
    },

    /// Update a DNS record by ID
    Update {
        /// Domain name (e.g. example.com)
        #[arg(long)]
        zone: String,

        /// DNS record ID to update
        #[arg(long)]
        id: String,

        /// Record type
        #[arg(long = "type")]
        record_type: Option<String>,

        /// Record name
        #[arg(long)]
        name: Option<String>,

        /// Record content
        #[arg(long)]
        content: Option<String>,

        /// TTL in seconds (1 = automatic)
        #[arg(long)]
        ttl: Option<u32>,

        /// Enable Cloudflare proxy (orange cloud)
        #[arg(long)]
        proxied: Option<bool>,
    },

    /// Delete a DNS record by ID
    Delete {
        /// Domain name (e.g. example.com)
        #[arg(long)]
        zone: String,

        /// DNS record ID to delete
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum AuthAction {
    /// Store an API key or token for a tool
    Set {
        /// Tool name
        tool: String,
        /// API key or token value
        key: String,
    },

    /// Retrieve the stored key for a tool (masked by default for AI safety)
    Get {
        /// Tool name
        tool: String,

        /// Show the full unmasked key (default: masked to prevent AI agent exposure)
        #[arg(long)]
        unmask: bool,
    },

    /// List all stored credentials (names only, values are masked)
    List,

    /// Remove a stored credential
    Remove {
        /// Tool name
        tool: String,
    },
}
