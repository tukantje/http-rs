use std::net::TcpListener;

const LISTENER_URL: &str = "127.0.0.1:42069";

fn main() {
    let listener = TcpListener::bind(LISTENER_URL).expect("error: failed to create TcpListener instance");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("info: accepted new connection");
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
    }
}
