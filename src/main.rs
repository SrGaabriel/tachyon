use crate::server::TachyonServer;

mod network;
mod packet;
mod protocol;
mod game;
mod server;
mod mojang;
mod security;

fn main() {
    let mut server = TachyonServer {
        keypair: security::generate_keypair().expect("Failed to generate keypair"),
        players: Default::default()
    };
    server.start();
}
