use std::fmt;

#[derive(Debug)]
pub enum StarfireError {
    UnknownTool(String),
    CredentialNotFound(String),
    CliNotFound {
        tool: String,
        binary: String,
        install_hint: String,
    },
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for StarfireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTool(t) => {
                writeln!(f, "unknown tool: '{t}'")?;
                writeln!(f)?;
                writeln!(f, "  Run 'starfire list' to see supported tools.")?;
                write!(f, "  Run 'starfire skill {t}' for usage details (if available).")
            }
            Self::CredentialNotFound(t) => {
                writeln!(f, "no credential stored for '{t}'")?;
                writeln!(f)?;
                writeln!(f, "  To fix, register your key:")?;
                writeln!(f, "    starfire register {t} <your-token>")?;
                writeln!(f)?;
                write!(f, "  Run 'starfire skill {t}' to see where to get your token.")
            }
            Self::CliNotFound { tool, binary, install_hint } => {
                writeln!(f, "'{binary}' is not installed (required for '{tool}')")?;
                writeln!(f)?;
                writeln!(f, "  Install it first:")?;
                writeln!(f, "    {install_hint}")?;
                writeln!(f)?;
                writeln!(f, "  Then register your credentials:")?;
                writeln!(f, "    starfire register {tool} <your-token>")?;
                writeln!(f)?;
                write!(f, "  Run 'starfire skill {tool}' for full setup instructions.")
            }
            Self::IoError(e) => write!(f, "I/O error: {e}"),
            Self::JsonError(e) => write!(f, "config error: {e}"),
        }
    }
}

impl From<std::io::Error> for StarfireError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::Error> for StarfireError {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}
