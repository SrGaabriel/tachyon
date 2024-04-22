use std::net::TcpStream;
use rsa::RsaPublicKey;
use uuid::Uuid;
use crate::game::entity::Player;

use crate::packet::Packet;
use crate::protocol::play::player_impls::ConnectingPlayer;
use crate::protocol::ProtocolState;

pub struct PlayerConnection {
    pub stream: TcpStream,
    pub state: ProtocolState,
    pub uuid: Option<Uuid>,
    pub compression: Option<i32>,
    pub connection_info: Option<ConnectionInfo>,
    pub security_info: Option<SecurityInfo>,
}

#[derive(Clone)]
pub struct ConnectionInfo {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16
}

#[derive(Clone)]
pub struct SecurityInfo {
    pub verify_token: Vec<u8>,
    pub public_key: RsaPublicKey
}

impl PlayerConnection {
    pub fn new(stream: TcpStream) -> Self {
        PlayerConnection {
            stream,
            state: ProtocolState::Handshaking,
            uuid: None,
            compression: None,
            connection_info: None,
            security_info: None
        }
    }

    pub fn dispatch(&mut self, packet: &mut Packet) {
        packet.write(&mut self.stream, self.compression.clone());
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
            compression: self.compression,
            security_info: self.security_info.clone(),
            connection_info: self.connection_info.clone(),
            uuid: self.uuid
        }
    }
}