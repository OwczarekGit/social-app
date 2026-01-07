use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct S3Config {
    #[arg(env("S3_URL"))]
    pub s3_url: String,
    #[arg(env("S3_USER"))]
    pub s3_user: String,
    #[arg(env("S3_PASSWORD"))]
    pub s3_password: String,
}
