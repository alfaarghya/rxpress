use std::collections::HashMap;

/// Represents an HTTP request.
///
/// Stores method, path, headers, query parameters, route parameters, and body.
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub version: String,
    pub query: HashMap<String, String>,
    pub params: HashMap<String, String>,
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
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("Content-Type".into(), "application/json".into());
    /// let req = Request::new("GET / HTTP/1.1", headers, "".into());
    ///
    /// assert_eq!(req.header("content-type"), Some(&"application/json".to_string()));
    /// ```
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    /// Gets a route parameter value (set by the router).
    ///
    /// # Example
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

    /// Gets a query parameter value.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use rxpress::Request;
    ///
    /// let headers = HashMap::new();
    /// let req = Request::new("GET /search?q=rust HTTP/1.1", headers, "".into());
    ///
    /// assert_eq!(req.query("q"), Some(&"rust".to_string()));
    /// ```
    pub fn query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
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

    #[test]
    fn test_parse_request_line_and_query() {
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost".into());

        let req = Request::new(
            "GET /hello?developer=alfaarghya HTTP/1.1",
            headers,
            "".into(),
        );

        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/hello");
        assert_eq!(req.version, "HTTP/1.1");
        assert_eq!(req.query("developer"), Some(&"alfaarghya".to_string()));
    }

    #[test]
    fn test_headers_case_insensitive() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "application/json".into());

        let req = Request::new("GET / HTTP/1.1", headers, "".into());
        assert_eq!(
            req.header("content-type"),
            Some(&"application/json".to_string())
        );
    }
}
