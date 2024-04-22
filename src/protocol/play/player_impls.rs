use uuid::Uuid;

use crate::game::{ChatMode, Gamemode};
use crate::game::entity::Player;
use crate::game::math::position::{Position, Rotation, Vector};
use crate::game::text::TextComponent;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition};
use crate::protocol::play::chat::ClientboundSystemMessagePacket;

#[derive(Clone)]
pub struct ConnectingPlayer {
    pub username: String,
    pub uuid: Uuid,
    pub connection: PlayerConnection
}

impl Player for ConnectingPlayer {
    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn get_entity_id(&self) -> i32 {
        panic!("Entity ID not set for connecting player")
    }

    fn is_chat_colored(&self) -> bool {
        panic!("Chat color not set for connecting player")
    }

    fn get_chat_mode(&self) -> ChatMode {
        panic!("Chat mode not set for connecting player")
    }

    fn get_view_distance(&self) -> i8 {
        panic!("View distance not set for connecting player")
    }

    fn get_locale(&self) -> String {
        panic!("Locale not set for connecting player")
    }

    fn is_righthanded(&self) -> bool {
        panic!("Righthanded not set for connecting player")
    }

    fn is_text_filtering_enabled(&self) -> bool {
        panic!("Text filtering not set for connecting player")
    }

    fn allows_server_listings(&self) -> bool {
        panic!("Server listings not set for connecting player")
    }

    fn get_gamemode(&self) -> Gamemode {
        panic!("Gamemode not set for connecting player")
    }

    fn set_gamemode(&mut self, gamemode: Gamemode) {
        panic!("Gamemode not set for connecting player")
    }

    fn get_connection(&self) -> &PlayerConnection {
        &self.connection
    }

    fn set_position(&mut self, position: Position) {
        panic!("Position not set for connecting player")
    }

    fn get_position(&self) -> &Position {
        panic!("Position not set for connecting player")
    }

    fn set_rotation(&mut self, rotation: Rotation) {
        panic!("Rotation not set for connecting player")
    }

    fn set_vector(&mut self, vector: Vector) {
        panic!("Vector not set for connecting player")
    }

    fn send_message(&mut self, message: TextComponent) {}

    fn send_packet(&mut self, packet: &mut Packet) {
        self.connection.dispatch(packet);
    }
}

pub struct InitialPlayer {
    pub username: String,
    pub uuid: Uuid,
    pub entity_id: i32,
    pub chat_colored: bool,
    pub chat_mode: ChatMode,
    pub view_distance: i8,
    pub locale: String,
    pub righthanded: bool,
    pub text_filtering_enabled: bool,
    pub server_listings_enabled: bool,
    pub gamemode: Gamemode,
    pub connection: PlayerConnection,
    pub position: Position
}

impl Player for InitialPlayer {
    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn get_entity_id(&self) -> i32 {
        self.entity_id
    }

    fn is_chat_colored(&self) -> bool {
        self.chat_colored
    }

    fn get_chat_mode(&self) -> ChatMode {
        self.chat_mode
    }

    fn get_view_distance(&self) -> i8 {
        self.view_distance
    }

    fn get_locale(&self) -> String {
        self.locale.clone()
    }

    fn is_righthanded(&self) -> bool {
        self.righthanded
    }

    fn is_text_filtering_enabled(&self) -> bool {
        self.text_filtering_enabled
    }

    fn allows_server_listings(&self) -> bool {
        self.server_listings_enabled
    }

    fn get_gamemode(&self) -> Gamemode {
        self.gamemode
    }

    fn set_gamemode(&mut self, gamemode: Gamemode) {
        self.gamemode = gamemode;
    }

    fn get_connection(&self) -> &PlayerConnection {
        &self.connection
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_rotation(&mut self, rotation: Rotation) {
        self.set_position(
            Position::from_vector_and_rotation(
                self.position.as_vector().clone(),
                rotation,
                self.position.on_ground
            )
        )
    }

    fn set_vector(&mut self, vector: Vector) {
        self.set_position(
            Position::from_vector(
                vector,
                self.position.yaw,
                self.position.pitch,
                self.position.on_ground
            )
        )
    }

    fn send_message(&mut self, message: TextComponent) {
        let mut packet = ClientboundSystemMessagePacket {
            message: message.into(),
            overlay: false
        };
        self.connection.dispatch(&mut packet.to_packet());
    }

    fn send_packet(&mut self, packet: &mut Packet) {
        self.connection.dispatch(packet);
    }
}