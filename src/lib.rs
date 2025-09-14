//! # rxpress
//!
//! `rxpress` is a minimal HTTP server framework inspired by Express.js, written in Rust.
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
//!
//! ## Route Parameters
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.get("/users/:id", |req, res| {
//!         if let Some(id) = req.param("id") {
//!             res.send(&format!("User ID: {}", id));
//!         } else {
//!             res.status(400).send("Missing ID");
//!         }
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ## Query Parameters
//!
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     app.get("/search", |req, res| {
//!         if let Some(query) = req.query("q") {
//!             res.send(&format!("Searching for: {}", query));
//!         } else {
//!             res.status(400).send("Missing query parameter `q`");
//!         }
//!     });
//!
//!     app.run();
//! }
//! ```
//!
//! ## Custom Headers
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
//! ## Multiple Methods
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

pub mod request;
pub mod response;
pub mod route;
pub mod router;
pub mod server;

pub use request::Request;
pub use response::Response;
pub use server::Server;
