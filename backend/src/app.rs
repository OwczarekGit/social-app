use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::SysRes;

pub struct App {
    listener: TcpListener,
}

impl App {
    pub async fn new(port: u16) -> SysRes<Self> {
        info!("Binding socket on port: {port}");
        Ok(Self {
            listener: TcpListener::bind(&format!("0.0.0.0:{port}")).await?,
        })
    }

    pub async fn run(self, routes: Router) -> SysRes<()> {
        info!("Starting server...");
        Ok(axum::serve(self.listener, routes).await?)
    }
}
