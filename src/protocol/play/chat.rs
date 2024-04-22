use crate::{define_packet, text};
use crate::game::MessageType;
use crate::game::text::TextComponent;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition};
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;
use crate::packet::types::vec::FixedSizeArray;
use crate::protocol::{ProtocolHandler, ProtocolState};
use crate::protocol::play::{ClientboundPlayerInfoUpdatePacket, PlayerAction};
use crate::server::TachyonServer;

pub struct ChatProtocolHandler;

impl ProtocolHandler for ChatProtocolHandler {
    fn ids(&self) -> Vec<i32> {
        vec![0x05]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Play
    }

    fn handle_packet(&self, server: &mut TachyonServer, connection: &mut PlayerConnection, packet: &mut Packet) {
        let uuid = connection.uuid.expect("Connection UUID not set");
        let player = server.players.get(&uuid).expect("Player not found");
        let packet = ServerboundChatMessagePacket::read_data(&mut packet.data).unwrap();

        println!("Player {} sent message: {}", player.get_username(), packet.message);
        server.broadcast_message(
            text!(format!("<{}> {}", player.get_username(), packet.message)),
            MessageType::Player
        );
    }
}

define_packet!(0x64, ClientboundSystemMessagePacket {
    message: TextComponent,
    overlay: bool
});

pub struct ServerboundChatMessagePacket {
    message: String,
    timestamp: i64,
    salt: i64,
    signature: Option<String>,
    message_count: i32,
    acknowledged: Vec<u8>
}

impl PacketDefinition for ServerboundChatMessagePacket {
    fn get_id() -> i32 {
        0x05
    }

    fn read_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let message = String::from_packet_data(buffer)?;
        let timestamp = i64::from_packet_data(buffer)?;
        let salt = i64::from_packet_data(buffer)?;
        let has_signature = bool::from_packet_data(buffer)?;
        let signature = if has_signature {
            Some(String::from_packet_data(buffer)?)
        } else {
            None
        };
        let message_count = VarInt::from_packet_data(buffer)?.value;
        let acknowledged = FixedSizeArray::<u8>::from_packet_data(buffer)?.data;
        Ok(
            ServerboundChatMessagePacket {
                message,
                timestamp,
                salt,
                signature,
                message_count,
                acknowledged
            }
        )
    }
}