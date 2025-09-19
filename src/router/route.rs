use crate::http::request::Request;
use crate::server::Handler;

/// Represents a single route definition (method + path + handler).
pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Handler,
}

impl Route {
    /// Creates a new [`Route`].
    pub fn new(method: &str, path: &str, handler: Handler) -> Route {
        Route {
            method: method.to_string(),
            path: path.to_string(),
            handler,
        }
    }

    /// Checks if this route matches a given request.
    ///
    /// Supports path parameters like `/users/:id`.
    pub fn matches(&self, req: &mut Request) -> bool {
        if self.method != req.method.to_uppercase() {
            return false;
        }

        let route_parts: Vec<&str> = self.path.split('/').collect();
        let req_parts: Vec<&str> = req.path.split('/').collect();

        if route_parts.len() != req_parts.len() {
            return false;
        }

        for (r, p) in route_parts.iter().zip(req_parts.iter()) {
            if r.starts_with(':') {
                // store param
                let key = r.trim_start_matches(':').to_string();
                if !p.is_empty() {
                    req.params.insert(key, p.to_string());
                }
                println!("[params]: {:?}", req.params);
            } else if r != p {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn dummy_handler(_req: &Request, _res: &mut crate::Response) {}

    #[test]
    fn test_route_match_static() {
        let route = Route::new("GET", "/hello", dummy_handler);
        let req = Request::new("GET /hello HTTP/1.1", HashMap::new(), "".into());
        let mut req = req;
        assert!(route.matches(&mut req));
    }

    #[test]
    fn test_route_match_with_param() {
        let route = Route::new("GET", "/users/:id", dummy_handler);
        let req = Request::new("GET /users/123 HTTP/1.1", HashMap::new(), "".into());
        let mut req = req;
        assert!(route.matches(&mut req));
        assert_eq!(req.param("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_route_with_missing_param() {
        let route = Route::new("GET", "/users/:id", dummy_handler);
        let req = Request::new("GET /users/ HTTP/1.1", HashMap::new(), "".into());
        let mut req = req;
        assert!(route.matches(&mut req));
        assert_eq!(req.param("id"), None);
    }
}
