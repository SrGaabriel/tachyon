use std::net::SocketAddr;

use crate::network::TcpServer;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::login::LoginProtocolHandler;
use crate::protocol::ProtocolHandler;
use crate::protocol::status::StatusRequestHandler;

mod network;
mod packet;
mod protocol;
mod game;

fn main() {
    let mut server = TcpServer::new(SocketAddr::from(([127, 0, 0, 1], 25565)));
    server.register_handler(Box::new(HandshakeRequestHandler::new()));
    server.register_handler(Box::new(StatusRequestHandler::new()));
    server.register_handler(Box::new(LoginProtocolHandler::new()));
    server.start();
}
