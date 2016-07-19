extern crate protobuf;

mod server;
mod renderers;
mod context;

fn main() {
    let mut server = server::Server::new();
    server.listen();
}
