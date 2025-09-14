use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

/// Represents an HTTP response.
///
/// Used by route handlers to set status codes, headers, and send body content.
pub struct Response<'a> {
    pub stream: &'a mut TcpStream,
    pub headers: HashMap<String, String>,
    pub status: u16,
}

impl<'a> Response<'a> {
    /// Creates a new [`Response`] object with default status `200 OK`.
    pub fn new(stream: &'a mut TcpStream) -> Response<'a> {
        let mut headers = HashMap::new();
        headers.insert("HTTP-Server-Powered-By".to_string(), "rxpress".to_string());

        Response {
            stream,
            headers,
            status: 200,
        }
    }

    /// Sets the HTTP status code.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Request, Response};
    /// # fn handler(_req: &Request, res: &mut Response) {
    /// res.status(404).send("Not found");
    /// # }
    /// ```
    pub fn status(&mut self, code: u16) -> &mut Self {
        self.status = code;
        self
    }

    /// Sets a header on the response.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Request, Response};
    /// # fn handler(_req: &Request, res: &mut Response) {
    /// res.set_header("X-Custom", "1234");
    /// res.send("ok");
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
    /// # use rxpress::{Request, Response};
    /// # fn handler(_req: &Request, res: &mut Response) {
    /// res.send("Hello, plain text!");
    /// # }
    /// ```
    pub fn send(&mut self, msg: &str) {
        self.set_header("Content-Type", "text/plain");
        self.write_response(msg.as_bytes());
    }

    /// Sends a JSON response with `Content-Type: application/json`.
    ///
    /// # Example
    /// ```
    /// # use rxpress::{Request, Response};
    /// # fn handler(_req: &Request, res: &mut Response) {
    /// res.json(r#"{"message":"ok"}"#);
    /// # }
    /// ```
    pub fn json(&mut self, msg: &str) {
        self.set_header("Content-Type", "application/json");
        self.write_response(msg.as_bytes());
    }

    /*---- Private Functions ----*/
    fn write_response(&mut self, msg: &[u8]) {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n");

        // println!("[write_response]: {headers:?}");
        let res = format!(
            "HTTP/1.1 {} OK\r\n{}\r\nContent-Length: {}\r\n\r\n",
            self.status,
            headers,
            msg.len()
        );

        self.stream.write_all(res.as_bytes()).unwrap();
        self.stream.write_all(msg).unwrap();
        self.stream.flush().unwrap();
    }
}
