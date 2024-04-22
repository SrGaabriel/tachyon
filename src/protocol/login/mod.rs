pub mod encryption;

use std::fs::File;
use std::io::Read;

use uuid::Uuid;

use crate::define_packet;
use crate::game::identifier::NamespaceId;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition, ParsePacketError};
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;
use crate::protocol::{ProtocolHandler, ProtocolState};
use crate::protocol::login::encryption::enable_encryption;
use crate::protocol::play::player_impls::ConnectingPlayer;
use crate::server::TachyonServer;

pub(crate) struct LoginProtocolHandler {
    pub encrypted: bool
}

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
    previous_gamemode: i8,
    dimension_names: Vec<String>,
    registry_codec: File,
    dimension_type: NamespaceId,
    dimension_name: NamespaceId,
    hashed_seed: i64,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    respawn_screen: bool,
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool
});

impl ProtocolHandler for LoginProtocolHandler {
    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Login
    }

    fn handle_packet(&self, server: &mut TachyonServer, connection: &mut PlayerConnection, packet: &mut Packet) {
        let login_start = ServerboundLoginStartPacket::read_data(&mut packet.data)
            .expect("Failed to read login start packet");
        let uuid = login_start.uuid.unwrap_or(
            Uuid::new_v3(&Uuid::NAMESPACE_DNS, format!("OfflinePlayer:{}", login_start.username).as_bytes())
        );
        connection.uuid = Some(uuid);

        let player = Box::new(ConnectingPlayer {
            username: login_start.username.clone(),
            uuid,
            connection: connection.clone()
        });
        server.players.insert(uuid, player);

        if !self.encrypted {
            allow_login(uuid, login_start.username, connection);
            return;
        }
        enable_encryption(server, connection);
    }
}

pub(crate) fn allow_login(uuid: Uuid, username: String, connection: &mut PlayerConnection) {
    let login_success = ClientboundLoginSuccessPacket {
        uuid,
        username,
        properties: vec![]
    };

    connection.dispatch(&mut login_success.to_packet());
    connection.state = ProtocolState::Play;

    let join_game = create_join_packet();
    connection.dispatch(&mut join_game.to_packet());
}

fn create_join_packet() -> ClientboundJoinGamePacket {
    let mut registry_codec = File::open("assets/registry_codec.nbt").unwrap();

    ClientboundJoinGamePacket {
        entity_id: 0,
        is_hardcore: false,
        gamemode: 0,
        previous_gamemode: -1,
        dimension_names: vec![],
        registry_codec,
        dimension_type: NamespaceId::new("minecraft", "overworld"),
        dimension_name: NamespaceId::new("minecraft", "world"),
        hashed_seed: 0,
        max_players: VarInt::from(20),
        view_distance: VarInt::from(10),
        simulation_distance: VarInt::from(10),
        reduced_debug_info: false,
        respawn_screen: true,
        is_debug: false,
        is_flat: false,
        has_death_location: false
    }
}