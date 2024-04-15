use std::net::TcpStream;
use crate::packet::Packet;
use crate::packet::types::PacketStructure;
use crate::protocol::ProtocolState;

pub struct PlayerConnection {
    pub stream: TcpStream,
    pub state: ProtocolState,
    pub connection_info: Option<ConnectionInfo>
}

pub struct ConnectionInfo {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
}

impl PlayerConnection {
    pub fn new(stream: TcpStream) -> Self {
        PlayerConnection {
            stream,
            state: ProtocolState::Handshaking,
            connection_info: None
        }
    }

    pub fn dispatch(&mut self, packet: &mut Packet) {
        packet.write(&mut self.stream);
    }
}