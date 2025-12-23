use anyhow::{Context, Result};
use std::io::Write;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    const RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";

    stream
        .write_all(RESPONSE.as_bytes())
        .context("failed to write to TcpStream")?;
    stream.flush().context("failed to flush TcpStream")?;

    Ok(())
}
