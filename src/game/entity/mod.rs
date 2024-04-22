use uuid::Uuid;

use crate::game::{ChatMode, Gamemode};
use crate::game::math::position::{Position, Rotation, Vector};
use crate::game::text::TextComponent;
use crate::network::connection::PlayerConnection;
use crate::packet::Packet;

pub trait Player: Send + Sync {
    fn get_username(&self) -> String;

    fn get_uuid(&self) -> Uuid;

    fn get_entity_id(&self) -> i32;

    fn is_chat_colored(&self) -> bool;

    fn get_chat_mode(&self) -> ChatMode;

    fn get_view_distance(&self) -> i8;

    fn get_locale(&self) -> String;

    fn is_righthanded(&self) -> bool;

    fn is_text_filtering_enabled(&self) -> bool;

    fn allows_server_listings(&self) -> bool;

    fn get_gamemode(&self) -> Gamemode;

    fn set_gamemode(&mut self, gamemode: Gamemode);

    fn get_connection(&self) -> &PlayerConnection;

    fn set_position(&mut self, position: Position);

    fn get_position(&self) -> &Position;

    fn set_rotation(&mut self, rotation: Rotation);

    fn set_vector(&mut self, vector: Vector);

    fn send_message(&mut self, message: TextComponent);

    fn send_packet(&mut self, packet: &mut Packet);
}