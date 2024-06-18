use std::error::Error;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn launch_game() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes())?;

    // let mut buffer = [0; 512];
    // stream.write_all(b"helloooooo this is the server speaking!")?;
    // stream.read_exact(&mut buffer).unwrap();
    // dbg!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // stream.write

    // let ships: [Ship; 5] = [Ship {
    //     start: Vec2i { x: 0, y: 0 },
    //     end: Vec2i { x: 0, y: 1 },
    // }; 5];

    // let board = Board::new(ships);

    Ok(())
}
