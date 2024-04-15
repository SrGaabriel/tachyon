use crate::network::TcpServer;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::ProtocolHandler;
use crate::protocol::status::StatusRequestHandler;

mod network;
mod packet;
mod protocol;

fn main() {
    let mut server = TcpServer::new("127.0.0.1:25565".to_string());
    server.register_handler(Box::new(HandshakeRequestHandler::new()));
    server.register_handler(Box::new(StatusRequestHandler::new()));
    server.start();
}
