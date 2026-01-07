use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Subcommand)]
pub enum StartSubcommand {
    CreateAdminAccount(CreateAdminAccount),
}

#[derive(Serialize, Deserialize, Debug, Clone, Args)]
pub struct CreateAdminAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}
