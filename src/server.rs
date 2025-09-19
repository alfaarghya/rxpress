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

use std::net::TcpListener;

use crate::connection::handle_connection;
use crate::http::request::Request;
use crate::http::response::Response;
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
    log_msg: Option<String>,
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
            log_msg: None,
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
    pub fn run(&mut self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind port");

        match &self.log_msg {
            Some(msg) => {
                println!("{}", msg);
                self.log_msg = None;
            }
            None => println!("[rxpress] running on http://{} ⚙️", self.address),
        }

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle_connection(stream, &self.router),
                Err(err) => eprintln!("Connection failed: {}", err),
            }
        }
    }

    /// Receive messages from user that can be logged later
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// fn main() {
    ///     let mut app = Server::new("3000");
    ///
    ///     app.log("Server is up!!").run(); // Now server can print custom log
    /// }
    /// ```
    pub fn log(&mut self, msg: &str) -> &mut Self {
        self.log_msg = Some(msg.to_string());
        self
    }

    /// Returns the server's full address (`127.0.0.1:<port>`).
    pub fn address(&self) -> &str {
        &self.address
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
