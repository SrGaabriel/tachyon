use std::io::Read;

use uuid::Uuid;

use crate::define_packet;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition};
use crate::packet::types::boolean::MinecraftBoolean;
use crate::packet::types::byte::{MinecraftByte, MinecraftUnsignedByte};
use crate::packet::types::file::MinecraftFile;
use crate::packet::types::PacketStructure;
use crate::packet::types::string::MinecraftString;
use crate::packet::types::uuid::MinecraftUuid;
use crate::packet::types::varint::MinecraftInt;
use crate::packet::types::vec::MinecraftVec;
use crate::protocol::{ProtocolHandler, ProtocolState};

pub(crate) struct LoginProtocolHandler;

pub struct ServerboundLoginStartPacket {
    username: MinecraftString,
    uuid: Option<MinecraftUuid>
}

impl PacketDefinition for ServerboundLoginStartPacket {
    fn get_id() -> i32 {
        0x00
    }

    fn read_data(buffer: &mut dyn Read) -> Self {
        let username = MinecraftString::read(buffer);
        let has_uuid = MinecraftBoolean::read(buffer);
        let uuid = if has_uuid.value {
            Some(MinecraftUuid::read(buffer))
        } else {
            None
        };
        ServerboundLoginStartPacket {
            username,
            uuid
        }
    }
}

define_packet!(0x02, ClientboundLoginSuccessPacket {
    uuid: MinecraftUuid,
    username: MinecraftString,
    properties: MinecraftVec<String, MinecraftString>
});

define_packet!(0x28, ClientboundJoinGamePacket {
    entity_id: MinecraftInt,
    is_hardcore: MinecraftBoolean,
    gamemode: MinecraftUnsignedByte,
    previous_gamemode: MinecraftByte,
    dimension_names: MinecraftVec<String, MinecraftString>,
    registry_codec: MinecraftFile
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
        let login_start = ServerboundLoginStartPacket::read_data(&mut packet.data);
        let uuid = match login_start.uuid {
            Some(uuid) => uuid.value,
            None => Uuid::new_v3(&Uuid::NAMESPACE_DNS, format!("OfflinePlayer:{}", login_start.username.value).as_bytes())
        };
        let login_success = ClientboundLoginSuccessPacket {
            uuid: MinecraftUuid::from(uuid),
            username: login_start.username,
            properties: MinecraftVec::from(vec![])
        };
        connection.dispatch(&mut login_success.to_packet());
        connection.state = ProtocolState::Play;

        // Send join game packet

    }
}