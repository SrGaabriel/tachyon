use crate::define_packet;
use crate::network::connection::{ConnectionInfo, PlayerConnection};
use crate::packet::{Packet, PacketDefinition};
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;
use crate::protocol::{ProtocolHandler, ProtocolState};
use crate::server::TachyonServer;

define_packet!(0x00, ServerboundHandshakePacket {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt
});

pub(crate) struct HandshakeRequestHandler;

impl ProtocolHandler for HandshakeRequestHandler {
    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Handshaking
    }

    fn handle_packet(&self, server: &mut TachyonServer, connection: &mut PlayerConnection, packet: &mut Packet) {
        let handshake_packet = ServerboundHandshakePacket::read_data(&mut packet.data)
            .expect("Failed to read handshake packet");
        connection.connection_info = Some(ConnectionInfo {
            protocol_version: handshake_packet.protocol_version.into(),
            server_address: handshake_packet.server_address,
            server_port: handshake_packet.server_port
        });
        connection.state = ProtocolState::from_id(handshake_packet.next_state.into());
    }
}