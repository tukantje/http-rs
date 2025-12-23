use std::{net::TcpListener, thread, time::Duration};

use anyhow::{Context, Result};

#[test]
fn test_server_responds_200_ok() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").context("could not create TcpListener")?;
    let addr = listener
        .local_addr()
        .context("could not retrieve TcpListener address")?;

    thread::spawn(move || {
        let stream = listener
            .incoming()
            .next()
            .expect("expected an incoming connection")
            .expect("failed to accept connection");
        http_rs::handle_connection(stream).expect("failed to handle connection");
    });

    thread::sleep(Duration::from_millis(50));

    let response = reqwest::blocking::get(format!("http://{}", addr))
        .context("could not retrieve response from server")?;

    assert_eq!(response.status(), 200);
    Ok(())
}

#[test]
fn test_server_response_404_not_found() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").context("could not create TcpListener")?;
    let addr = listener
        .local_addr()
        .context("could not retrieve TcpListener address")?;

    thread::spawn(move || {
        let stream = listener
            .incoming()
            .next()
            .expect("expected an incoming connection")
            .expect("failed to accept connection");
        http_rs::handle_connection(stream).expect("failed to handle connection");
    });

    thread::sleep(Duration::from_millis(50));

    let response = reqwest::blocking::get(format!("http://{}/thispathdoesnotexist", addr))
        .context("could not retrieve response from server")?;

    assert_eq!(response.status(), 404);
    Ok(())
}
