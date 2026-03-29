use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AuthType {
    /// API token (scoped, generated from dashboard)
    ApiToken,
    /// Personal access token
    Pat,
    /// API key (typically a long-lived secret key)
    ApiKey,
    /// OAuth token (obtained via OAuth flow)
    OAuthToken,
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiToken => write!(f, "API Token"),
            Self::Pat => write!(f, "Personal Access Token"),
            Self::ApiKey => write!(f, "API Key"),
            Self::OAuthToken => write!(f, "OAuth Token"),
        }
    }
}

#[allow(dead_code)]
pub struct ToolDef {
    pub name: &'static str,
    pub description: &'static str,
    /// Install hint shown in error messages (not executed by starfire)
    pub install_cmd: &'static str,
    /// Binary name on $PATH
    pub binary_name: &'static str,
    /// Environment variable name for the credential
    pub env_var: &'static str,
    /// Type of authentication this tool uses
    pub auth_type: AuthType,
    /// Human-readable label for the credential
    pub auth_label: &'static str,
}

pub fn default_registry() -> HashMap<&'static str, ToolDef> {
    let mut m = HashMap::new();

    m.insert("wrangler", ToolDef {
        name: "wrangler",
        description: "Cloudflare Workers CLI",
        install_cmd: "npm install -g wrangler",
        binary_name: "wrangler",
        env_var: "CLOUDFLARE_API_TOKEN",
        auth_type: AuthType::ApiToken,
        auth_label: "Cloudflare API Token",
    });

    m.insert("flarectl", ToolDef {
        name: "flarectl",
        description: "Cloudflare DNS & zone mgmt",
        install_cmd: "go install github.com/cloudflare/cloudflare-go/cmd/flarectl@latest",
        binary_name: "flarectl",
        env_var: "CF_API_TOKEN",
        auth_type: AuthType::ApiToken,
        auth_label: "Cloudflare API Token",
    });

    m.insert("cloudflared", ToolDef {
        name: "cloudflared",
        description: "Cloudflare Tunnel CLI",
        install_cmd: "See https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/",
        binary_name: "cloudflared",
        env_var: "TUNNEL_TOKEN",
        auth_type: AuthType::ApiToken,
        auth_label: "Cloudflare Tunnel Token",
    });

    m.insert("railway", ToolDef {
        name: "railway",
        description: "Railway deployment CLI",
        install_cmd: "npm install -g @railway/cli",
        binary_name: "railway",
        env_var: "RAILWAY_TOKEN",
        auth_type: AuthType::Pat,
        auth_label: "Railway Project Token",
    });

    m.insert("fal", ToolDef {
        name: "fal",
        description: "fal.ai CLI for serverless AI",
        install_cmd: "pip install fal",
        binary_name: "fal",
        env_var: "FAL_KEY",
        auth_type: AuthType::ApiKey,
        auth_label: "fal.ai API Key",
    });

    m.insert("vercel", ToolDef {
        name: "vercel",
        description: "Vercel deployment CLI",
        install_cmd: "npm install -g vercel",
        binary_name: "vercel",
        env_var: "VERCEL_TOKEN",
        auth_type: AuthType::Pat,
        auth_label: "Vercel Personal Access Token",
    });

    m.insert("flyctl", ToolDef {
        name: "flyctl",
        description: "Fly.io CLI",
        install_cmd: "curl -L https://fly.io/install.sh | sh",
        binary_name: "flyctl",
        env_var: "FLY_API_TOKEN",
        auth_type: AuthType::ApiToken,
        auth_label: "Fly.io API Token",
    });

    m.insert("supabase", ToolDef {
        name: "supabase",
        description: "Supabase CLI",
        install_cmd: "npm install -g supabase",
        binary_name: "supabase",
        env_var: "SUPABASE_ACCESS_TOKEN",
        auth_type: AuthType::Pat,
        auth_label: "Supabase Access Token",
    });

    m.insert("neonctl", ToolDef {
        name: "neonctl",
        description: "NeonDB serverless Postgres CLI",
        install_cmd: "npm install -g neonctl",
        binary_name: "neonctl",
        env_var: "NEON_API_KEY",
        auth_type: AuthType::ApiKey,
        auth_label: "Neon API Key",
    });

    m.insert("netlify", ToolDef {
        name: "netlify",
        description: "Netlify CLI",
        install_cmd: "npm install -g netlify-cli",
        binary_name: "netlify",
        env_var: "NETLIFY_AUTH_TOKEN",
        auth_type: AuthType::Pat,
        auth_label: "Netlify Personal Access Token",
    });

    m.insert("better-auth", ToolDef {
        name: "better-auth",
        description: "BetterAuth authentication CLI",
        install_cmd: "npm install better-auth",
        binary_name: "better-auth",
        env_var: "BETTER_AUTH_SECRET",
        auth_type: AuthType::ApiKey,
        auth_label: "BetterAuth Secret Key",
    });

    m
}
