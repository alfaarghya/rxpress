use crate::request::Request;
use crate::response::Response;
use crate::route::Route;
use crate::server::Handler;

/// Router manages all registered routes and dispatches requests.
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Creates a new, empty [`Router`].
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    /// Adds a new route with method, path, and handler.
    pub fn add_route(&mut self, method: &str, path: &str, handler: Handler) {
        self.routes.push(Route::new(method, path, handler));
    }

    /// Dispatches a request to the first matching route handler.
    pub fn handle(&self, req: &mut Request, res: &mut Response) {
        for route in &self.routes {
            if route.matches(req) {
                (route.handler)(req, res);
                return;
            }
        }

        res.status(404).send("404 Not Found");
    }
}
