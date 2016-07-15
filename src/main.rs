extern crate protobuf;

mod context;
mod server;
mod renderers;

fn main() {
    let mut ctx = context::Context::new();
    let mut server = server::Server::new(&mut ctx);
    server.listen();
}
