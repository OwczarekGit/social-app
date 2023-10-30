use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Subcommand)]
pub enum ExecuteActionOnStart {
    CreateAdminAccount(CreateAdminAccount)
}

#[derive(Serialize, Deserialize, Debug, Clone, Args)]
pub struct  CreateAdminAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub create_admin_args: Option<ExecuteActionOnStart>,
}
