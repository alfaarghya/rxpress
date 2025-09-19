//! # Parser Module
//! This module contains some helper functions that parse request URL, headers & body

use std::collections::HashMap;
use std::io::{BufReader, Lines, Read};
use std::net::TcpStream;

/// Read the request-line (e.g., "GET /path HTTP/1.1")
pub fn get_request_line(
    lines: &mut Lines<&mut BufReader<&TcpStream>>,
) -> Result<String, &'static str> {
    match lines.next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => {
            eprintln!("[rxpress error]: failed to read request line: {}", e);
            Err("failed to read request line")
        }
        None => {
            eprintln!("[rxpress error]: client disconnected before sending request line");
            Err("no request line")
        }
    }
}

/// Read headers into a HashMap (lowercased keys)
pub fn get_headers(lines: &mut Lines<&mut BufReader<&TcpStream>>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    while let Some(line_result) = lines.next() {
        match line_result {
            Ok(line) => {
                if line.is_empty() {
                    break;
                }
                if let Some((key, val)) = line.split_once(":") {
                    map.insert(key.trim().to_ascii_lowercase(), val.trim().to_string());
                } else {
                    eprintln!("[rxpress warning]: malformed header '{}'", line);
                }
            }
            Err(_) => break, // ignore malformed header line instead of panicking
        }
    }

    map
}

/// Read body given headers (supports Content-Length)
pub fn get_body(
    headers: &HashMap<String, String>,
    buf_reader: &mut BufReader<&TcpStream>,
) -> Result<String, &'static str> {
    // find the body with 'content-length' key
    if let Some(len) = headers.get("content-length") {
        if let Ok(size) = len.parse::<usize>() {
            let mut buffer = vec![0; size];
            return match buf_reader.read_exact(&mut buffer) {
                Ok(_) => Ok(String::from_utf8_lossy(&buffer).to_string()),
                Err(e) => {
                    eprintln!("[rxpress error]: failed to read body: {}", e);
                    Err("failed to read body")
                }
            };
        }
    }

    Ok(String::new()) // no body
}
