use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use crate::http::parser::{get_body, get_headers, get_request_line};
use crate::http::request::Request;
use crate::http::response::Response;
use crate::router::Router;

// Handles an incoming client connection.
///
/// Reads a basic HTTP request and responds with a static message.
pub fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.by_ref().lines();

    //Request URL
    let request_line = match get_request_line(&mut lines) {
        Ok(line) => line,
        Err(_) => return,
    };

    //Request Headers
    let headers = get_headers(&mut lines);

    // Body
    let body = match get_body(&headers, &mut buf_reader) {
        Ok(b) => b,
        Err(_) => return,
    };

    let mut req = Request::new(&request_line, headers, body);
    let mut res = Response::new(&mut stream);

    router.handle(&mut req, &mut res);
}
