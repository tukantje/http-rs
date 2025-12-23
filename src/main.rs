use std::net::TcpListener;

use anyhow::{Context, Result};

const LISTENER_URL: &str = "127.0.0.1:0";

fn main() -> Result<()> {
    let listener =
        TcpListener::bind(LISTENER_URL).context("failed to create TcpListener instance")?;
    println!(
        "info: server started at http://{}",
        listener
            .local_addr()
            .context("could not retrieve address for TcpListener instance")?
    );

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("error: {}", error);
                continue;
            }
        };

        println!("info: accepted new connection");
        if let Err(error) = http_rs::handle_connection(stream) {
            eprintln!("error: failed to handle connection: {}", error)
        };
    }

    Ok(())
}
