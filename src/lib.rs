//! # rxpress
//!
//! `rxpress` is a minimal HTTP server framework inspired by Express.js, written in Rust.
//!
//! ## Features
//! - Define routes for all major HTTP methods (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)
//! - Route parameters and query parameters support
//! - Custom response headers and status codes
//! - Minimalistic, synchronous design
//!
//! ## Quick Start
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("8080");
//!
//!     // A simple GET route
//!     app.get("/", |_req, res| {
//!         res.send("Hello from rxpress!");
//!     });
//!
//!     // A POST route
//!     app.post("/submit", |_req, res| {
//!         res.status(201).json(r#"{"status":"created"}"#);
//!     });
//!
//!     app.run(); // blocks forever
//! }
//! ```
//! ## Module Overview
//! - `request` - Defines the [`Request`] struct for accessing request data.
//! - `response` - Defines the [`Response`] struct for sending responses.
//! - `route` - Defines a single route with path, method, and handler.
//! - `router` - Handles route registration and request dispatching.
//! - `server` - The main [`Server`] struct to run the HTTP server.
//! - `status` - Standard HTTP status codes as [`HttpStatus`] enum.
//!
//! ## Request Helpers
//! The [`Request`] struct provides convenient helpers for
//! accessing headers, query parameters, path parameters, body, and more.
//!
//! ### Headers
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     // header()
//!     app.get("/headers", |req, res| {
//!         if let Some(ua) = req.header("User-Agent") {
//!             res.send(&format!("Your User-Agent: {}", ua));
//!         } else {
//!             res.send("User-Agent not found");
//!         }
//!     });
//!
//!     // header_or()
//!     app.get("/host", |req, res| {
//!         let host = req.header_or("Host", "localhost");
//!         res.send(&format!("Host: {}", host));
//!     });
//!
//!     // header_expect()
//!     app.get("/auth", |req, res| {
//!         match req.header_expect("Authorization") {
//!             Ok(token) => res.send(&format!("Token: {}", token)),
//!             Err(err) => res.status(400).send(&err),
//!         }
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ## Route Parameters
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     // param()
//!     app.get("/users/:id", |req, res| {
//!         if let Some(id) = req.param("id") {
//!             res.send(&format!("User ID: {}", id));
//!         } else {
//!             res.send("Missing user ID");
//!         }
//!     });
//!
//!     // param_or()
//!     app.get("/items/:id", |req, res| {
//!         let user_id = req.param_or("id", "0");
//!         res.send(&format!("User ID (or default): {}", user_id));
//!     });
//!
//!     // param_expect()
//!     app.get("/secure/:id", |req, res| {
//!         match req.param_expect("id") {
//!             Ok(id) => res.send(&format!("Secure User ID: {}", id)),
//!             Err(err) => res.status(400).send(&err),
//!         }
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ### Query Parameters
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     // query()
//!     app.get("/search", |req, res| {
//!         if let Some(q) = req.query("q") {
//!             res.send(&format!("Searching for: {}", q));
//!         } else {
//!             res.send("Missing query param `q`");
//!         }
//!     });
//!
//!     // query_or()
//!     app.get("/history", |req, res| {
//!         let query = req.query_or("q", "none");
//!         res.send(&format!("Query (or default): {}", query));
//!     });
//!
//!     // query_expect()
//!     app.get("/secure_search", |req, res| {
//!         match req.query_expect("q") {
//!             Ok(q) => res.send(&format!("Secure search for: {}", q)),
//!             Err(err) => res.status(400).send(&err),
//!         }
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ### Custom Headers
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.get("/headers", |_req, res| {
//!         res.set_header("X-Custom-Header", "rxpress")
//!            .send("Custom header sent!");
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ## Response Helper
//!
//!  The [`Response`] struct represents the outgoing HTTP response.  
//! It allows setting headers, status codes, and sending text, JSON, or HTML responses.
//!
//! ### Custom Status Examples
//!
//! Demonstrates using `status()` with enum, numeric codes, and custom reason.
//! Only the first response (`send()` or `json()`) will be sent; subsequent calls
//! will print a warning and be ignored.
//!
//! #### Example 1: Standard Enum
//! ```no_run
//! # use rxpress::{Server, HttpStatus};
//! # fn main() {
//! let mut app = Server::new("3000");
//!
//! app.get("/error_enum", |_req, res| {
//!     res.status(HttpStatus::Forbidden).send("Forbidden!");
//!     res.send("Ignored response"); // Will print warning
//! });
//!
//! app.run();
//! # }
//! ```
//!
//! #### Example 2: Custom Code
//! ```no_run
//! # use rxpress::Server;
//! # fn main() {
//! let mut app = Server::new("3000");
//!
//! app.get("/error_code", |_req, res| {
//!     res.status(511).json(r#"{"error":"Network Auth Required"}"#);
//!     res.json(r#"{"ignored": true}"#); // Will print warning
//! });
//!
//! app.run();
//! # }
//! ```
//!
//! #### Example 3: Custom Code + Reason
//! ```no_run
//! # use rxpress::Server;
//! # fn main() {
//! let mut app = Server::new("3000");
//!
//! app.get("/error_custom", |_req, res| {
//!     res.status((599, "Network Timeout")).send("Timeout!");
//!     res.send("Ignored"); // Will print warning
//! });
//!
//! app.run();
//! # }
//! ```
//!
//! ### HTML Responses
//!
//! You can use `html()` to send inline HTML with the proper `Content-Type`.
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.get("/page", |_req, res| {
//!         res.html("<h1>Hello from rxpress!</h1><p>This is HTML.</p>");
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ### Serving HTML Files
//!
//! Use `html_file()` to serve HTML from disk. If the file is missing or unreadable,
//! a `500 Internal Server Error` is automatically returned.
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.get("/home", |_req, res| {
//!         res.html_file("index.html");
//!     });
//!
//!     app.get("/about", |_req, res| {
//!         res.html_file("about.html");
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ### Multiple Methods
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.put("/update", |_req, res| {
//!         res.send("Resource updated!");
//!     });
//!
//!     app.delete("/delete", |_req, res| {
//!         res.send("Resource deleted!");
//!     });
//!
//!     app.patch("/patch", |_req, res| {
//!         res.send("Resource patched!");
//!     });
//!
//!     app.options("/any", |_req, res| {
//!         res.set_header("Allow", "GET, POST, OPTIONS");
//!         res.send("Allowed methods: GET, POST, OPTIONS");
//!     });
//!
//!     app.run();
//! }
//! ```

mod connection;
mod http;
mod router;
mod server;

pub use http::request::Request;
pub use http::response::Response;
pub use http::status::HttpStatus;
pub use server::Server;
