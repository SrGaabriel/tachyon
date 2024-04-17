use crate::network::TcpServer;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::ProtocolHandler;
use crate::protocol::status::StatusRequestHandler;

mod network;
mod packet;
mod protocol;
mod time;
mod server;

fn main() {
    let mut server = server::TachyonServer::new("Tachyon".to_string(), "127.0.0.1:25565".to_string());
    server.start();
}
