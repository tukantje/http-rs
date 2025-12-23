use std::net::TcpListener;

use anyhow::{Context, Result};
use tracing_subscriber::EnvFilter;

const LISTENER_URL: &str = "127.0.0.1:0";

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let listener =
        TcpListener::bind(LISTENER_URL).context("failed to create TcpListener instance")?;

    match listener.local_addr() {
        Ok(addr) => tracing::info!(%addr, "server started"),
        Err(error) => tracing::error!(%error, "failed to get server address"),
    }

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                tracing::error!(%error, "connection error");
                continue;
            }
        };

        tracing::info!("accepted new connection");
        if let Err(error) = http_rs::handle_connection(stream) {
            tracing::error!(%error, "failed to handle connection");
        };
    }

    Ok(())
}
