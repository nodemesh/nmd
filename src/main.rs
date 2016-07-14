extern crate protobuf;

pub mod context;
pub mod server;

fn main() {
    let mut ctx = context::Context::new();
    let mut server = server::Server::new(&mut ctx);
    server.listen()
}
