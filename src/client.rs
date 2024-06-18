use std::io::{self, prelude::*};
use std::net::TcpStream;

pub fn join_game() -> Result<(), Box<dyn std::error::Error>> {
    // Rust is a secure programming language.
    // I'm not though:
    print!("What's the IP address of the Host you're connecting to? default: 127.0.0.1:7878: ");
    io::stdout().flush()?;

    let mut ip_addr_input = String::new();
    io::stdin().read_line(&mut ip_addr_input)?;

    if ip_addr_input.trim().is_empty() {
        ip_addr_input = String::from("127.0.0.1:7878");
    }

    loop {
        let mut stream = TcpStream::connect(ip_addr_input.trim())?;

        // let msg = b"Hello, my name is client!";
        // stream.write_all(msg)?;

        print!("enter a chat msg: ");
        io::stdout().flush()?;
        let mut chat = String::new();
        io::stdin().read_line(&mut chat)?;

        stream.write_all(chat.as_bytes())?;

        // let mut buf = [0; 128];
        // stream.read_exact(&mut buf)?;

        // let msg = String::from_utf8_lossy(&buf);
        // println!("{msg}");
    }
    // Ok(())
}
