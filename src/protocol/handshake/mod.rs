use crate::define_packet;
use crate::network::connection::{ConnectionInfo, PlayerConnection};
use crate::packet::Packet;
use crate::packet::types::numbers::MinecraftUnsignedShort;
use crate::packet::types::PacketStructure;
use crate::packet::types::string::MinecraftString;
use crate::packet::types::varint::MinecraftVarInt;
use crate::protocol::{ProtocolHandler, ProtocolState};

define_packet!(0x00, HandshakePacket {
    protocol_version: MinecraftVarInt,
    server_address: MinecraftString,
    server_port: MinecraftUnsignedShort,
    next_state: MinecraftVarInt
});

pub(crate) struct HandshakeRequestHandler;

impl ProtocolHandler for HandshakeRequestHandler {
    fn new() -> Self {
        HandshakeRequestHandler
    }

    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Handshaking
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection) {
        let handshake_packet = HandshakePacket::read(&mut packet.data);
        connection.connection_info = Some(ConnectionInfo {
            protocol_version: handshake_packet.protocol_version.into(),
            server_address: handshake_packet.server_address.into(),
            server_port: handshake_packet.server_port.into()
        });
        connection.state = ProtocolState::from_id(handshake_packet.next_state.into());
    }
}