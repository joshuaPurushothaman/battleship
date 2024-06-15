use std::io::prelude::*;
// use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn launch_game() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        dbg!("Connection established!");
        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read_exact(&mut buffer).unwrap();

    dbg!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // stream.write
}