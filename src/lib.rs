use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use tracing::instrument;

const RESPONSE_200_OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_404_NOT_FOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

#[instrument(skip(stream))]
pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut buffer = Vec::new();

    loop {
        reader.read_until(b'\n', &mut buffer)?;

        if buffer.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    let request_str = String::from_utf8_lossy(&buffer);
    let request = request_str.lines().collect::<Vec<&str>>();

    anyhow::ensure!(!request.is_empty(), "empty request received");

    let first_line = request[0];

    // Example
    // GET /index.html HTTP/1.1
    // Hence, we need the second token after splitting via whitespace
    let path = first_line
        .split_whitespace()
        .nth(1)
        .context("no path in request")?;

    let response = match path {
        "/" => RESPONSE_200_OK,
        _ => RESPONSE_404_NOT_FOUND,
    };

    stream
        .write_all(response)
        .context("failed to write to TcpStream")?;
    stream.flush().context("failed to flush TcpStream")?;

    Ok(())
}
