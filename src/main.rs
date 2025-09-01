use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind port");

    println!("Listening on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Request received:\n{}", String::from_utf8_lossy(&buffer));

            let response =
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from rxpress server!";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(err) => eprintln!("Failed to read from connection: {}", err),
    }
}
