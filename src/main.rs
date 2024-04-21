use crate::server::TachyonServer;

mod network;
mod packet;
mod protocol;
mod game;
mod server;

fn main() {
    let mut server = TachyonServer {
        players: Default::default()
    };
    server.start();
}
