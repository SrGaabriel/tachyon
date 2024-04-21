use std::net::TcpStream;
use uuid::Uuid;
use crate::game::entity::Player;

use crate::packet::Packet;
use crate::protocol::play::player_impls::ConnectingPlayer;
use crate::protocol::ProtocolState;

pub struct PlayerConnection {
    pub stream: TcpStream,
    pub state: ProtocolState,
    pub connection_info: Option<ConnectionInfo>,
    pub uuid: Option<Uuid>
}

#[derive(Clone)]
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
            connection_info: None,
            uuid: None
        }
    }

    pub fn dispatch(&mut self, packet: &mut Packet) {
        packet.write(&mut self.stream);
    }

    pub fn disconnect(&mut self, reason: &str) {
    }

    pub fn close_gracefully(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

impl Clone for PlayerConnection {
    fn clone(&self) -> Self {
        PlayerConnection {
            stream: self.stream.try_clone().expect("Failed to clone stream"),
            state: self.state,
            connection_info: self.connection_info.clone(),
            uuid: self.uuid
        }
    }
}