use std::io::{Read, Write};
use uuid::Uuid;
use crate::define_packet;
use crate::game::{ChatMode, Gamemode};
use crate::game::entity::Player;
use crate::game::math::position::{Position, Vector};
use crate::game::text::TextComponent;
use crate::mojang::SkinProperty;
use crate::network::connection::PlayerConnection;
use crate::network::TcpServer;
use crate::packet::{Packet, PacketDefinition, ParsePacketError};
use crate::packet::types::PacketStructure;
use crate::packet::types::position::EncodedVector;
use crate::packet::types::varint::VarInt;
use crate::protocol::{ProtocolHandler, ProtocolState};
use crate::protocol::play::player_impls::InitialPlayer;
use crate::protocol::play::position::ClientboundSetDefaultSpawnPositionPacket;
use crate::server::TachyonServer;

pub(crate) mod player_impls;
pub mod position;
pub mod chat;

pub fn register_all_handlers(server: &mut TcpServer) {
    server.register_handler(Box::new(PlayLoginProtocolHandler {}));
    server.register_handler(Box::new(chat::ChatProtocolHandler {}));
    server.register_handler(Box::new(position::PositionProtocolHandler {}));
}

pub(crate) struct PlayLoginProtocolHandler;

impl ProtocolHandler for PlayLoginProtocolHandler {
    fn ids(&self) -> Vec<i32> {
        vec![0x00, 0x08, 0x0D]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Play
    }

    fn handle_packet(&self, server: &mut TachyonServer, connection: &mut PlayerConnection, packet: &mut Packet) {
        println!("Play login protocol handler handling packet with id: {}", packet.id);
        let uuid = connection.uuid.expect("Player UUID not set");

        if packet.id == 0x08 {
            let client_information = ServerboundClientInformationPacket::read_data(&mut packet.data).unwrap();
            println!("Client information: {:?}", client_information);

            let initial_player = upgrade_player(
                server.players.get(&uuid).unwrap().as_ref(),
                client_information
            );
            let position = initial_player.position.clone();
            server.players.insert(uuid, Box::new(initial_player));

            let mut default_spawn = ClientboundSetDefaultSpawnPositionPacket {
                vector: EncodedVector::new(position.x, position.y, position.z),
                angle: (&position).yaw
            }.to_packet();
            connection.dispatch(&mut default_spawn);

            let mut player_info = ClientboundPlayerInfoUpdatePacket::actions(vec![
                PlayerAction::AddPlayer(
                    uuid,
                    server.players.get(&uuid).unwrap().get_username(),
                    vec![]
                ),
                PlayerAction::UpdateListed(
                    uuid,
                    true
                )
            ]);
            connection.dispatch(&mut player_info.to_packet());
            return
        }
    }
}

define_packet!(0x1B, ClientboundPlayDisconnectPacket {
    reason: String
});

define_packet!(0x08, ServerboundClientInformationPacket {
    locale: String,
    view_distance: i8,
    chat_mode: ChatMode,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listing: bool
});

define_packet!(0x3A, ClientboundPlayerInfoUpdatePacket {
    actions_mask: u8,
    actions: Vec<PlayerAction>
});

fn upgrade_player(connecting_player: &dyn Player, client_info_packet: ServerboundClientInformationPacket) -> InitialPlayer {
    InitialPlayer {
        username: connecting_player.get_username(),
        uuid: connecting_player.get_uuid(),
        entity_id: 1,
        chat_colored: client_info_packet.chat_colors,
        gamemode: Gamemode::Survival,
        connection: connecting_player.get_connection().clone(),
        righthanded: client_info_packet.main_hand.value == 1,
        locale: client_info_packet.locale,
        view_distance: client_info_packet.view_distance,
        chat_mode: client_info_packet.chat_mode,
        text_filtering_enabled: client_info_packet.enable_text_filtering,
        server_listings_enabled: client_info_packet.allow_server_listing,
        position: Position::override_vector(Vector { x: 0.0, y: 0.0, z: 0.0})
    }
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    AddPlayer(
        Uuid,
        String,
        Vec<SkinProperty>
    ),
    UpdateGamemode(
        Uuid,
        Gamemode
    ),
    UpdateListed(
        Uuid,
        bool
    ),
    UpdateLatency(
        Uuid,
        i32
    ),
    UpdateDisplayName(
        Uuid,
        Option<TextComponent>
    ),
}

impl PacketStructure for PlayerAction {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        panic!("PlayerAction cannot be read from packet data");
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        match self {
            PlayerAction::AddPlayer(uuid, name, properties) => {
                uuid.write_packet_data(buffer);
                name.write_packet_data(buffer);
                properties.write_packet_data(buffer)
            },
            PlayerAction::UpdateGamemode(uuid, gamemode) => {
                uuid.write_packet_data(buffer);
                gamemode.write_packet_data(buffer);
            },
            PlayerAction::UpdateListed(uuid, listed) => {
                uuid.write_packet_data(buffer);
                listed.write_packet_data(buffer);
            },
            PlayerAction::UpdateLatency(uuid, latency) => {
                uuid.write_packet_data(buffer);
                VarInt::from(*latency).write_packet_data(buffer);
            },
            PlayerAction::UpdateDisplayName(uuid, display_name) => {
                uuid.write_packet_data(buffer);
                match display_name {
                    Some(name) => {
                        true.write_packet_data(buffer);
                        name.write_packet_data(buffer);
                    },
                    None => {
                        false.write_packet_data(buffer);
                    }
                }
            }
        }
    }
}

impl ClientboundPlayerInfoUpdatePacket {
    pub fn actions(actions: Vec<PlayerAction>) -> Self {
        let mut actions_mask = 0;
        for action in &actions {
            match action {
                PlayerAction::AddPlayer(_, _, _) => actions_mask |= 1 << 0,
                PlayerAction::UpdateGamemode(_, _) => actions_mask |= 1 << 2,
                PlayerAction::UpdateListed(_, _) => actions_mask |= 1 << 3,
                PlayerAction::UpdateLatency(_, _) => actions_mask |= 1 << 4,
                PlayerAction::UpdateDisplayName(_, _) => actions_mask |= 1 << 5
            }
        }

        ClientboundPlayerInfoUpdatePacket {
            actions_mask,
            actions
        }
    }
}