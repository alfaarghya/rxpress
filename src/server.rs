use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// A simple HTTP server for handling requests.
///
/// # Example
/// ```no_run
/// use rxpress::Server;
///
/// fn main() {
///     let app = Server::new("8080");
///     app.run();
/// }
/// ```
pub struct Server {
    address: String,
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

        Server { address }
    }

    /// Starts listening for incoming TCP connections.
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

    /// Handles an incoming client connection.
    ///
    /// Reads a basic HTTP request and responds with a static message.
    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        println!("[request] {}", request_line);

        let response =
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from rxpress server!";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
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
