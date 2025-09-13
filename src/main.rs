use rxpress::Server;

fn main() {
    let app = Server::new("8080");
    app.run();
}
