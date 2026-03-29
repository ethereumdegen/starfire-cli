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
