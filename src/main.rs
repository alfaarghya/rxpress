use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    println!("Listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Request received:\n{}", String::from_utf8_lossy(&buffer));

            let response =
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from rxserver server!";
            stream.write_all(response.as_bytes()).unwrap();

            stream.flush().unwrap();
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }
}
