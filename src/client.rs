use std::io::{self, prelude::*};
use std::net::TcpStream;

pub fn join_game() -> Result<(), Box<dyn std::error::Error>> {
    // Rust is a secure programming language.
    // I'm not though:
    print!("What's the IP address of the Host you're connecting to? (example: 127.0.0.1:7878)");

    let mut ip_addr_input = String::new();
    io::stdin().read_line(&mut ip_addr_input)?;

    let mut stream = TcpStream::connect(ip_addr_input.trim())?;

    let msg = b"Hello, my name is client!";

    stream.write_all(msg)?;
    stream.read_exact(&mut [0; 128])?;

    Ok(())
}
