use std::fs::File;
use std::io::Read;

use uuid::Uuid;

use crate::define_packet;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition, ParsePacketError};
use crate::packet::types::PacketStructure;
use crate::protocol::{ProtocolHandler, ProtocolState};

pub(crate) struct LoginProtocolHandler;

pub struct ServerboundLoginStartPacket {
    username: String,
    uuid: Option<Uuid>
}

impl PacketDefinition for ServerboundLoginStartPacket {
    fn get_id() -> i32 {
        0x00
    }

    fn read_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let username = String::from_packet_data(buffer)?;
        let has_uuid = bool::from_packet_data(buffer)?;
        let uuid = if has_uuid {
            Some(Uuid::from_packet_data(buffer)?)
        } else {
            None
        };
        Ok(
            ServerboundLoginStartPacket {
                username,
                uuid
            }
        )
    }
}

define_packet!(0x02, ClientboundLoginSuccessPacket {
    uuid: Uuid,
    username: String,
    properties: Vec<String>
});

define_packet!(0x28, ClientboundJoinGamePacket {
    entity_id: i32,
    is_hardcore: bool,
    gamemode: u8,
    previous_gamemode: u8,
    dimension_names: Vec<String>,
    registry_codec: File
});

impl ProtocolHandler for LoginProtocolHandler {
    fn new() -> Self where Self: Sized {
        LoginProtocolHandler
    }

    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Login
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection) {
        println!("Login protocol handler handling packet with id: {}", packet.id);
        let login_start = ServerboundLoginStartPacket::read_data(&mut packet.data)
            .expect("Failed to read login start packet");
        let uuid = login_start.uuid.unwrap_or(
            Uuid::new_v3(&Uuid::NAMESPACE_DNS, format!("OfflinePlayer:{}", login_start.username).as_bytes())
        );
        let login_success = ClientboundLoginSuccessPacket {
            uuid,
            username: login_start.username,
            properties: vec![]
        };
        connection.dispatch(&mut login_success.to_packet());
        connection.state = ProtocolState::Play;

        // Send join game packet
    }
}