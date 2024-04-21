use crate::define_packet;
use crate::game::{ChatMode, Gamemode};
use crate::game::entity::Player;
use crate::game::math::position::{Position, Vector};
use crate::network::connection::PlayerConnection;
use crate::network::TcpServer;
use crate::packet::{Packet, PacketDefinition};
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