//! # Server Module
//!
//! This module provides the core [`Server`] struct, which manages the HTTP server,
//! registers routes, and dispatches requests to handlers.  
//! It is the main entrypoint when building apps with `rxpress`.
//!
//! ## Example
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("8080");
//!
//!     app.get("/", |_req, res| {
//!         res.send("Hello, world!");
//!     });
//!
//!     app.run(); // blocks forever
//! }
//! ```

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Lines, Read};
use std::net::{TcpListener, TcpStream};

use crate::request::Request;
use crate::response::Response;
use crate::router::Router;

/// Type alias for a request handler function.
///
/// Handlers receive the [`Request`] and a mutable reference to the [`Response`].
///
/// ```no_run
/// use rxpress::{Request, Response};
///
/// fn handler(_req: &Request, res: &mut Response) {
///     res.send("Hello!");
/// }
/// ```
pub type Handler = fn(&Request, &mut Response);

/// A simple HTTP server for handling requests.
///
/// The [`Server`] manages a [`Router`] internally, where routes are registered
/// using convenience methods such as [`Server::get`] or [`Server::post`].
///
/// # Example
/// ```no_run
/// use rxpress::Server;
///
/// fn main() {
///     let mut app = Server::new("8080");
///     app.get("/", |_req, res| {
///         res.send("Hello, world!");
///     });
///     app.run();
/// }
/// ```
pub struct Server {
    address: String,
    router: Router,
}

impl Server {
    /// Creates a new [`Server`] bound to `127.0.0.1:<port>`.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number to bind the server on.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    /// let server = Server::new("8080");
    /// assert_eq!(server.address(), "127.0.0.1:8080");
    /// ```
    pub fn new(port: &str) -> Server {
        let localhost: &str = "127.0.0.1";
        let address = format!("{}:{}", localhost, port);

        Server {
            address,
            router: Router::new(),
        }
    }

    /// Registers a handler for the GET method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.get("/", |_req, res| {
    ///         res.send("Hello from rxpress with GET method!");
    ///     });
    /// }
    /// ```
    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.add_route("GET", path, handler);
    }

    /// Registers a handler for the POST method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.post("/submit", |_req, res| {
    ///         res.json(r#"{"status":"ok"}"#);
    ///     });
    /// }
    /// ```
    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.add_route("POST", path, handler);
    }

    /// Registers a handler for the PUT method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.put("/update", |_req, res| {
    ///         res.send("Resource updated");
    ///     });
    /// }
    /// ```
    pub fn put(&mut self, path: &str, handler: Handler) {
        self.router.add_route("PUT", path, handler);
    }

    /// Registers a handler for the DELETE method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.delete("/remove", |_req, res| {
    ///         res.send("Deleted!");
    ///     });
    /// }
    /// ```
    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.router.add_route("DELETE", path, handler);
    }

    /// Registers a handler for the PATCH method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.patch("/modify", |_req, res| {
    ///         res.send("Patched!");
    ///     });
    /// }
    /// ```
    pub fn patch(&mut self, path: &str, handler: Handler) {
        self.router.add_route("PATCH", path, handler);
    }

    /// Registers a handler for the OPTIONS method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.options("/any", |_req, res| {
    ///         res.set_header("Allow", "GET, POST, OPTIONS");
    ///         res.send("Allowed methods listed");
    ///     });
    /// }
    /// ```
    pub fn options(&mut self, path: &str, handler: Handler) {
        self.router.add_route("OPTIONS", path, handler);
    }

    /// Registers a handler for the HEAD method at the given path.
    ///
    /// # Example
    /// ```
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.head("/check", |_req, res| {
    ///         res.set_header("Content-Length", "0");
    ///     });
    /// }
    /// ```
    pub fn head(&mut self, path: &str, handler: Handler) {
        self.router.add_route("HEAD", path, handler);
    }

    /// Starts listening for incoming TCP connections.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///     app.get("/", |_req, res| {
    ///         res.send("Hello world!");
    ///     });
    ///
    ///     app.run(); // blocks forever
    /// }
    /// ```
    ///
    /// This function will block the current thread until the server is stopped.
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind port");

        println!("[rxpress] running on http://{} ⚙️", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(err) => eprintln!("Connection failed: {}", err),
            }
        }
    }

    /// Returns the server's full address (`127.0.0.1:<port>`).
    pub fn address(&self) -> &str {
        &self.address
    }

    /* ---- Private Functions ---- */
    // Handles an incoming client connection.
    ///
    /// Reads a basic HTTP request and responds with a static message.
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&stream);
        let mut lines = buf_reader.by_ref().lines();

        //Request URL
        let request_line = match self.get_request_line(&mut lines) {
            Ok(line) => line,
            Err(_) => return,
        };
        // println!("[request] {}", request_line);

        //Request Headers
        let headers = self.get_headers(&mut lines);
        // println!("[headers] {:?}", headers);

        // Body
        let body = match self.get_body(&headers, &mut buf_reader) {
            Ok(b) => b,
            Err(_) => return,
        };
        // println!("[body] {}", body);

        let mut req = Request::new(&request_line, headers, body);
        let mut res = Response::new(&mut stream);

        // res.send("Hello from rxpress server!");
        // res.json(r#"{"message":"hello world"}"#);

        self.router.handle(&mut req, &mut res);
    }

    // get HTTP request(method, path, version)
    fn get_request_line(
        &self,
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

    //get all headers
    fn get_headers(
        &self,
        lines: &mut Lines<&mut BufReader<&TcpStream>>,
    ) -> HashMap<String, String> {
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

    //get complete body
    fn get_body(
        &self,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit test: Ensure that the server builds the address correctly.
    #[test]
    fn test_server_new() {
        let server = Server::new("3000");
        assert_eq!(server.address(), "127.0.0.1:3000");
    }
}
