//! # Request Module
//!
//! The [`Request`] struct represents an incoming HTTP request.  
//! It stores the request line, headers, body, query parameters, and path parameters.
//!
//! ## Example
//! ```no_run
//! use rxpress::Server;
//!
//! fn main() {
//!     let mut app = Server::new("3000");
//!
//!     // header() and header_or()
//!     app.get("/headers", |req, res| {
//!         // header() returns Option<&String>
//!         if let Some(ua) = req.header("user-agent") {
//!             res.send(&format!("Your User-Agent: {}", ua));
//!         } else {
//!             res.send("User-Agent header not found");
//!         }
//!         // header_or() returns default if header missing
//!         let host = req.header_or("host", "localhost");
//!         println!("Host: {}", host);
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
//!     // param() and param_or()
//!     app.get("/users/:id", |req, res| {
//!         // param() returns Option<&String>
//!         if let Some(id) = req.param("id") {
//!             res.send(&format!("User ID: {}", id));
//!         } else {
//!             res.send("Missing user ID");
//!         }
//!         // param_or() returns default if param missing
//!         let user_id = req.param_or("id", "0");
//!         println!("User ID (or default): {}", user_id);
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
//!     // query() and query_or()
//!     app.get("/search", |req, res| {
//!         // query() returns Option<&String>
//!         if let Some(q) = req.query("q") {
//!             res.send(&format!("Searching for: {}", q));
//!         } else {
//!             res.send("Missing query param `q`");
//!         }
//!         // query_or() returns default if query missing
//!         let query = req.query_or("q", "none");
//!         println!("Query (or default): {}", query);
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

use std::collections::HashMap;

/// Represents an HTTP request.
///
/// Stores method, path, headers, query parameters, route parameters, and body.
pub struct Request {
    /// HTTP method (e.g., `GET`, `POST`)
    pub method: String,
    /// Path portion of the request (e.g., `/users/123`)
    pub path: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// HTTP version (e.g., `HTTP/1.1`)
    pub version: String,
    /// Query parameters parsed into key-value pairs
    pub query: HashMap<String, String>,
    /// Path parameters extracted from route definitions
    pub params: HashMap<String, String>,
    /// Request body as a string
    pub body: String,
}

impl Request {
    /// Creates a new [`Request`] from raw parts.
    ///
    /// # Arguments
    /// * `request_line` - The first line of the HTTP request (`"GET /foo?bar=1 HTTP/1.1"`).
    /// * `headers` - The parsed HTTP headers.
    /// * `body` - The request body as a string.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let headers = HashMap::new();
    /// let req = Request::new("GET /hello?developer=alfaarghy HTTP/1.1", headers, "".to_string());
    /// assert_eq!(req.method, "GET");
    /// assert_eq!(req.path, "/hello");
    /// assert_eq!(req.query("developer"), Some(&"alfaarghy".to_string()));
    /// ```
    pub fn new(request_line: &str, headers: HashMap<String, String>, body: String) -> Request {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        let (method, full_path, version) = match parts.as_slice() {
            [m, p, v] => (m.to_string(), p.to_string(), v.to_string()),
            _ => ("GET".to_string(), "/".to_string(), "HTTP/1.1".to_string()),
        };

        let (path, query) = if let Some((p, q)) = full_path.split_once('?') {
            (p.to_string(), Self::parse_query(q))
        } else {
            (full_path, HashMap::new())
        };

        Request {
            method,
            path,
            headers,
            version,
            query,
            params: HashMap::new(),
            body,
        }
    }

    /// Gets a header value by key (case-insensitive).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/test", |req, res| {
    ///     if let Some(token) = req.header("Authorization") {
    ///         res.send(&format!("Token: {}", token));
    ///     } else {
    ///         res.status(400).send("Missing Authorization header");
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("Content-Type".into(), "application/json".into());
    /// let req = Request::new("GET / HTTP/1.1", headers, "".into());
    ///
    /// assert_eq!(req.header("content-type"), Some(&"application/json".to_string()));
    /// assert_eq!(req.header("non-existent"), None);
    /// ```
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    /// Gets a header value or returns a default if not present.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/test", |req, res| {
    ///     let content_type = req.header_or("Content-Type", "text/plain");
    ///     res.send(&format!("Content-Type: {}", content_type));
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let req = Request::new("GET / HTTP/1.1", HashMap::new(), "".into());
    /// assert_eq!(req.header_or("Content-Type", "text/plain"), "text/plain");
    /// ```
    pub fn header_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.headers
            .get(key)
            .map(|val| val.as_str())
            .unwrap_or(default)
    }

    /// Gets a header value or returns an error message if missing.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/test", |req, res| {
    ///     match req.header_expect("Authorization") {
    ///         Ok(token) => res.send(token),
    ///         Err(err) => res.status(400).send(&err),
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("Authorization".into(), "Bearer abc123".into());
    /// let req = Request::new("GET / HTTP/1.1", headers, "".into());
    ///
    /// assert_eq!(req.header_expect("Authorization").unwrap(), "Bearer abc123");
    /// assert!(req.header_expect("X-Token").is_err());
    /// ```
    pub fn header_expect(&self, key: &str) -> Result<&str, String> {
        self.headers.get(key).map(|val| val.as_str()).ok_or(format!(
            "[rxpress error]: Required header `{}` is missing. \
            Please include it in your request, e.g., `{}: value`.",
            key, key
        ))
    }

    /// Gets a route parameter value (set by the router).
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/users/:id", |req, res| {
    ///     if let Some(id) = req.param("id") {
    ///         res.send(&format!("User ID: {}", id));
    ///     } else {
    ///         res.status(400).send("Missing user ID");
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let mut req = Request::new("GET /users/42 HTTP/1.1", HashMap::new(), "".into());
    /// req.params.insert("id".into(), "42".into());
    ///
    /// assert_eq!(req.param("id"), Some(&"42".to_string()));
    /// ```
    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    /// Gets a route parameter or returns a default if missing.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/users/:id", |req, res| {
    ///     let id = req.param_or("id", "0");
    ///     res.send(&format!("User ID: {}", id));
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let req = Request::new("GET /users/ HTTP/1.1", HashMap::new(), "".into());
    /// assert_eq!(req.param_or("id", "0"), "0");
    /// ```
    pub fn param_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.params
            .get(key)
            .map(|val| val.as_str())
            .unwrap_or(default)
    }

    /// Gets a route parameter or returns an error message if missing.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/users/:id", |req, res| {
    ///     match req.param_expect("id") {
    ///         Ok(id) => res.send(&format!("User ID: {}", id)),
    ///         Err(err) => res.status(400).send(&err),
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let mut req = Request::new("GET /users/42 HTTP/1.1", HashMap::new(), "".into());
    /// req.params.insert("id".into(), "42".into());
    /// assert_eq!(req.param_expect("id").unwrap(), "42");
    /// assert!(req.param_expect("username").is_err());
    /// ```
    pub fn param_expect(&self, key: &str) -> Result<&str, String> {
        self.params.get(key).map(|val| val.as_str()).ok_or(format!(
            "[rxpress error]: Required route parameter `{}` is missing. \
            Ensure your route includes it, e.g., `/route/:{}`.",
            key, key
        ))
    }

    /// Gets a query parameter value.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/search", |req, res| {
    ///     if let Some(q) = req.query("q") {
    ///         res.send(&format!("Searching for: {}", q));
    ///     } else {
    ///         res.status(400).send("Missing query parameter `q`");
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let headers = HashMap::new();
    /// let req = Request::new("GET /search?q=rust HTTP/1.1", headers, "".into());
    /// assert_eq!(req.query("q"), Some(&"rust".to_string()));
    /// ```
    pub fn query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    /// Gets a query parameter or returns a default if missing.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/search", |req, res| {
    ///     let q = req.query_or("q", "none");
    ///     res.send(&format!("Query: {}", q));
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let req = Request::new("GET /search HTTP/1.1", HashMap::new(), "".into());
    /// assert_eq!(req.query_or("q", "none"), "none");
    /// ````
    pub fn query_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.query
            .get(key)
            .map(|val| val.as_str())
            .unwrap_or(default)
    }

    /// Gets a query parameter or returns an error message if missing.
    ///
    /// # Example
    /// ```no_run
    /// use rxpress::Server;
    ///
    /// let mut app = Server::new("8080");
    ///
    /// app.get("/search", |req, res| {
    ///     match req.query_expect("q") {
    ///         Ok(val) => res.send(val),
    ///         Err(err) => res.status(400).send(&err),
    ///     }
    /// });
    /// ```
    /// ---
    /// ## Test
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let req = Request::new("GET /search?q=rust HTTP/1.1", HashMap::new(), "".into());
    /// assert_eq!(req.query_expect("q").unwrap(), "rust");
    /// assert!(req.query_expect("page").is_err());
    /// ```
    pub fn query_expect(&self, key: &str) -> Result<&str, String> {
        self.query.get(key).map(|val| val.as_str()).ok_or(format!(
            "[rxpress error]: Required query parameter `{}` is missing. \
            Please include it in your request, e.g., `/route?{}=value`.",
            key, key
        ))
    }

    /*---- Private Functions ----*/
    /// Parses query parameters into a [`HashMap`].
    fn parse_query(q: &str) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();

        for pair in q.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                map.insert(k.to_string(), v.to_string());
            } else {
                map.insert(pair.to_string(), "".to_string());
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_req_line(line: &str) -> Request {
        Request::new(line, HashMap::new(), "".into())
    }

    // TEST - header test
    #[test]
    fn test_header_case_insensitive() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "application/json".into());
        let req = Request::new("GET / HTTP/1.1", headers, "".into());

        assert_eq!(
            req.header("content-type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_header_or() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "application/json".into());

        let req = Request::new("GET / HTTP/1.1", headers, "".into());

        assert_eq!(
            req.header_or("Content-Type", "text/plain"),
            "application/json"
        );
        assert_eq!(req.header_or("Non-Existent", "default"), "default");
    }

    #[test]
    fn test_header_expect() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".into(), "Bearer abc123".into());

        let req = Request::new("GET / HTTP/1.1", headers, "".into());

        // Existing header
        assert_eq!(req.header_expect("Authorization").unwrap(), "Bearer abc123");

        // Missing header
        let err = req.header_expect("X-Token").unwrap_err();
        assert!(err.contains("Required header `X-Token` is missing"));
    }

    //TEST - params test
    #[test]
    fn test_param_insertion_and_lookup() {
        let mut req = make_req_line("GET /users/1 HTTP/1.1");
        req.params.insert("id".into(), "1".into());
        assert_eq!(req.param("id"), Some(&"1".to_string()));
    }

    #[test]
    fn test_param_or() {
        let mut req = Request::new("GET /users/ HTTP/1.1", HashMap::new(), "".into());
        req.params.insert("id".into(), "42".into());

        assert_eq!(req.param_or("id", "0"), "42");
        assert_eq!(req.param_or("username", "guest"), "guest");
    }

    #[test]
    fn test_param_expect() {
        let mut req = Request::new("GET /users/42 HTTP/1.1", HashMap::new(), "".into());
        req.params.insert("id".into(), "42".into());

        // Existing param
        assert_eq!(req.param_expect("id").unwrap(), "42");

        // Missing param
        let err = req.param_expect("username").unwrap_err();
        assert!(err.contains("Required route parameter `username` is missing"));
    }

    //TEST - query test
    #[test]
    fn test_query_lookup() {
        let req = make_req_line("GET /search?q=rust HTTP/1.1");
        assert_eq!(req.query("q"), Some(&"rust".to_string()));
    }

    #[test]
    fn test_query_or() {
        let req = Request::new("GET /search?q=rust HTTP/1.1", HashMap::new(), "".into());

        assert_eq!(req.query_or("q", "none"), "rust");
        assert_eq!(req.query_or("page", "1"), "1");
    }

    #[test]
    fn test_query_expect() {
        let req = Request::new("GET /search?q=rust HTTP/1.1", HashMap::new(), "".into());

        // Existing query
        assert_eq!(req.query_expect("q").unwrap(), "rust");

        // Missing query
        let err = req.query_expect("page").unwrap_err();
        assert!(err.contains("Required query parameter `page` is missing"));
    }

    //TEST - query parser(Private Method)
    #[test]
    fn test_parse_query_function() {
        let parsed = Request::parse_query("a=1&b=2&empty");
        assert_eq!(parsed.get("a"), Some(&"1".to_string()));
        assert_eq!(parsed.get("b"), Some(&"2".to_string()));
        assert_eq!(parsed.get("empty"), Some(&"".to_string()));
    }
}
