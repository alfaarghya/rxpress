//! # Response Module
//!
//! The [`Response`] struct is used to construct and send HTTP responses back to clients.
//! It supports setting headers, status codes, sending plain text or JSON, and ensures a response
//! is sent only once per request.
//!
//! ## Example
//! ```no_run
//! use std::net::TcpStream;
//! use rxpress::Response;
//!
//! fn handler(stream: &mut TcpStream) {
//!     let mut res = Response::new(stream);
//!     res.set_header("X-Custom", "rxpress")
//!        .status(201)
//!        .json(r#"{"message":"Created"}"#);
//! }
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::net::TcpStream;

use crate::http::status::{HttpStatus, StatusArg};

/// Represents an HTTP response.
///
/// Used by route handlers to set status codes, headers, and send body content.
pub struct Response<'a> {
    stream: &'a mut TcpStream,
    headers: HashMap<String, String>,
    status: HttpStatus,
    status_code: u16,
    status_reason: String,
    sent: bool,
}

impl<'a> Response<'a> {
    /// Creates a new Response with default `200 OK`.
    pub fn new(stream: &'a mut TcpStream) -> Response<'a> {
        let mut headers = HashMap::new();
        headers.insert("HTTP-Server-Powered-By".to_string(), "rxpress".to_string());

        Response {
            stream,
            headers,
            status: HttpStatus::OK,
            status_code: 200,
            status_reason: "OK".to_string(),
            sent: false,
        }
    }

    /// Sets the HTTP status.
    ///
    /// Supports `HttpStatus` enum, numeric codes, or custom code + reason.
    ///
    /// # Example (Standard Enum)
    /// ```no_run
    /// # use rxpress::{Response, HttpStatus};
    /// # fn handler(res: &mut Response) {
    /// res.status(HttpStatus::Forbidden).send("Forbidden!");
    /// # }
    /// ```
    ///
    /// # Example (Custom Code)
    /// ```no_run
    /// # use rxpress::Response;
    /// # fn handler(res: &mut Response) {
    /// res.status(511).json(r#"{"error":"Network Auth Required"}"#);
    /// # }
    /// ```
    ///
    /// # Example (Custom Code + Reason)
    /// ```no_run
    /// # use rxpress::Response;
    /// # fn handler(res: &mut Response) {
    /// res.status((599, "Network Timeout")).send("Timeout!");
    /// # }
    /// ```
    ///
    /// # Example (Multiple Calls)
    /// ```no_run
    /// # use rxpress::{Response, HttpStatus};
    /// # fn handler(res: &mut Response) {
    /// // Only the first send() is executed
    /// res.status(HttpStatus::OK).send("First response");
    /// res.json(r#"{"ignored": true}"#); // Will print warning and be ignored
    /// # }
    /// ```
    pub fn status<'b, T: Into<StatusArg<'b>>>(&mut self, arg: T) -> &mut Self {
        match arg.into() {
            StatusArg::Enum(e) => {
                self.status = e;
                self.status_code = e.code();
                self.status_reason = HttpStatus::reason(self.status_code).to_string();
            }
            StatusArg::Code(code) => {
                self.status_code = code;
                self.status_reason = HttpStatus::reason(code).to_string();
            }
            StatusArg::CodeReason(code, reason) => {
                self.status_code = code;
                self.status_reason = reason.to_string();
            }
        }

        self
    }

    /// Sets a header on the response.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Response};
    /// # fn handler(res: &mut Response) {
    /// res.set_header("X-Custom", "1234").send("ok");
    /// # }
    /// ```
    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Sends a plain text response with `Content-Type: text/plain`.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Response};
    /// # fn handler(res: &mut Response) {
    /// res.send("Hello, plain text!");
    /// res.send("Ignored"); // will print warning
    /// # }
    /// ```
    pub fn send(&mut self, msg: &str) {
        if self.sent {
            eprintln!(
                "[rxpress warning!]: response already sent, ignoring subsequent send() call."
            );
            return;
        }
        self.sent = true; // mark as sent
        self.set_header("Content-Type", "text/plain");
        self.write_response(msg.as_bytes());
    }

    /// Sends a JSON response with `Content-Type: application/json`.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Response};
    /// # fn handler(res: &mut Response) {
    /// res.json(r#"{"message":"ok"}"#);
    /// res.json(r#"{"ignored": true}"#); // will print warning
    /// # }
    /// ```
    pub fn json(&mut self, msg: &str) {
        if self.sent {
            eprintln!(
                "[rxpress warning!]: response already sent, ignoring subsequent json() call."
            );
            return;
        }
        self.sent = true; // mark as sent
        self.set_header("Content-Type", "application/json");
        self.write_response(msg.as_bytes());
    }

    /// Sends an HTML response with `Content-Type: text/html; charset=utf-8`.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/page", |_, res| {
    ///     res.html("<h1>Hello</h1><p>HTML response</p>");
    /// });
    /// ```
    pub fn html(&mut self, body: &str) {
        if self.sent {
            eprintln!(
                "[rxpress warning!]: response already sent, ignoring subsequent html() call."
            );
            return;
        }
        self.sent = true; // mark as sent
        self.set_header("Content-Type", "text/html; charset=utf-8");
        self.write_response(body.as_bytes());
    }

    /// Sends the contents of an HTML file with `Content-Type: text/html; charset=utf-8`.
    /// If the file cannot be read, responds with `500 Internal Server Error`.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/home", |_, res| {
    ///     res.html_file("index.html");
    /// });
    /// ```
    pub fn html_file(&mut self, path: &str) {
        if self.sent {
            eprintln!(
                "[rxpress warning!]: response already sent, ignoring subsequent html_file() call."
            );
            return;
        }
        self.sent = true; // mark as sent
        self.set_header("Content-Type", "text/html; charset=utf-8");
        match fs::read_to_string(path) {
            Ok(content) => {
                self.write_response(content.as_bytes());
            }
            Err(_) => {
                let body = &format!(
                    "<h2>Internal Server Error</h2>\n<p>No file found on {}</p>",
                    path
                );
                self.status(HttpStatus::InternalServerError);
                self.write_response(body.as_bytes());
            }
        }
    }

    /*---- Private Functions ----*/
    /// Send header & response message
    fn write_response(&mut self, msg: &[u8]) {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n");

        // println!("[write_response]: {headers:?}");
        let res = format!(
            "HTTP/1.1 {} {}\r\n{}\r\nContent-Length: {}\r\n\r\n",
            self.status_code,
            self.status_reason,
            headers,
            msg.len()
        );

        if let Err(e) = self.stream.write_all(res.as_bytes()) {
            eprintln!("[rxpress error]: failed to write headers: {}", e);
            return;
        }

        if let Err(e) = self.stream.write_all(msg) {
            eprintln!("[rxpress error]: failed to write body: {}", e);
            return;
        }

        if let Err(e) = self.stream.flush() {
            eprintln!("[rxpress error]: failed to flush stream: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};

    // helper to get a connected TcpStream pair
    fn tcp_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let client = TcpStream::connect(addr).unwrap();
        let server = listener.accept().unwrap().0;

        (client, server)
    }

    // TEST - set custom header
    #[test]
    fn test_set_header() {
        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.set_header("X-Test", "123");
        assert_eq!(res.headers.get("X-Test"), Some(&"123".to_string()));
    }

    // TEST - sent status from HttpStatus enum
    #[test]
    fn test_status_with_enum() {
        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.status(HttpStatus::Forbidden);
        assert_eq!(res.status_code, 403);
        assert_eq!(res.status_reason, "Forbidden");
    }

    // TEST - sent status code with custom reason
    #[test]
    fn test_status_with_custom_code_and_reason() {
        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.status((499, "Custom Reason"));
        assert_eq!(res.status_code, 499);
        assert_eq!(res.status_reason, "Custom Reason");
    }

    // TEST - send test/plain response OR application/json response
    #[test]
    fn test_send_and_json_set_content_type() {
        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.send("hello");
        assert_eq!(
            res.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );

        let (_c2, mut s2) = tcp_pair();
        let mut res2 = Response::new(&mut s2);
        res2.json(r#"{"msg":"ok"}"#);
        assert_eq!(
            res2.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    // TEST - html response sets proper content type
    #[test]
    fn test_html_sets_content_type() {
        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.html("<h1>Test</h1>");
        assert!(res.sent);
        assert_eq!(
            res.headers.get("Content-Type"),
            Some(&"text/html; charset=utf-8".to_string())
        );
    }

    // TEST - html_file loads file contents
    #[test]
    fn test_html_file_success_and_failure() {
        // success case
        let tmp_file = "test_html_file.html";
        fs::write(tmp_file, "<h1>Hello</h1>").unwrap();

        let (_c, mut s) = tcp_pair();
        let mut res = Response::new(&mut s);
        res.html_file(tmp_file);
        assert!(res.sent);
        assert_eq!(
            res.headers.get("Content-Type"),
            Some(&"text/html; charset=utf-8".to_string())
        );

        fs::remove_file(tmp_file).unwrap();

        // failure case (missing file → 500)
        let (_c2, mut s2) = tcp_pair();
        let mut res2 = Response::new(&mut s2);
        res2.html_file("missing_file.html");
        assert!(res2.sent);
        assert_eq!(res2.status_code, 500);
    }
}
