use crate::network::connection::PlayerConnection;
use crate::packet::Packet;
use crate::protocol::{ProtocolHandler, ProtocolState};

pub(crate) struct PlayLoginProtocolHandler;

impl ProtocolHandler for PlayLoginProtocolHandler {
    fn new() -> Self where Self: Sized {
        PlayLoginProtocolHandler
    }

    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Play
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection) {

    }
}