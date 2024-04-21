use crate::define_packet;
use crate::game::math::position::{Position, Rotation, Vector};
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition};
use crate::protocol::{ProtocolHandler, ProtocolState};
use crate::server::TachyonServer;
use crate::packet::types::PacketStructure;
use crate::packet::types::position::EncodedVector;

pub(crate) struct PositionProtocolHandler;

impl ProtocolHandler for PositionProtocolHandler {
    fn ids(&self) -> Vec<i32> {
        vec![0x14, 0x15, 0x16, 0x17]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Play
    }

    fn handle_packet(&self, server: &mut TachyonServer, connection: &mut PlayerConnection, packet: &mut Packet) {
        let player = server.players.get_mut(&connection.uuid.expect("Connection UUID not set")).expect("Player not found");

        match packet.id {
            0x14 => {
                let packet = ServerboundSetPlayerPositionPacket::read_data(&mut packet.data).unwrap();
                if packet.vector == player.get_position().as_vector() {
                    return;
                }
                if packet.vector.x.abs() > 3.2e7 || packet.vector.z.abs() > 3.2e7 {
                    connection.close_gracefully();
                    return;
                }
                player.set_vector(packet.vector);
            },
            0x15 => {
                let packet = ServerboundSetPlayerPositionAndRotationPacket::read_data(&mut packet.data).unwrap();
                player.set_position(packet.position);
            },
            0x16 => {
                let packet = ServerboundSetPlayerRotationPacket::read_data(&mut packet.data).unwrap();
                player.set_rotation(packet.rotation);
            },
            0x17 => {
                let packet = ServerboundSetPlayerOnGroundPacket::read_data(&mut packet.data).unwrap();
                // player.set_on_ground(packet.on_ground);
            },
            _ => ()
        }
    }
}

define_packet!(0x14, ServerboundSetPlayerPositionPacket {
    vector: Vector,
    on_ground: bool
});

define_packet!(0x15, ServerboundSetPlayerPositionAndRotationPacket {
    position: Position
});

define_packet!(0x16, ServerboundSetPlayerRotationPacket {
    rotation: Rotation,
    on_ground: bool
});

define_packet!(0x17, ServerboundSetPlayerOnGroundPacket {
    on_ground: bool
});

define_packet!(0x50, ClientboundSetDefaultSpawnPositionPacket {
    vector: EncodedVector,
    angle: f32
});