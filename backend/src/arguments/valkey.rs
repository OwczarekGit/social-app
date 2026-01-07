use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct ValkeyConfig {
    #[arg(env("VALKEY_URL"))]
    pub valkey_url: String,
}
