use std::cmp::Ordering;

use crate::network::connection::PlayerConnection;
use crate::packet::Packet;

pub mod status;
pub mod handshake;
pub mod login;
mod play;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProtocolState {
    Handshaking,
    Status,
    Login,
    Play
}

impl ProtocolState {
    fn from_id(id: i32) -> ProtocolState {
        match id {
            0 => ProtocolState::Handshaking,
            1 => ProtocolState::Status,
            2 => ProtocolState::Login,
            3 => ProtocolState::Play,
            _ => panic!("Invalid protocol state id: {}", id)
        }
    }
}

pub trait ProtocolHandler {
    fn new() -> Self where Self: Sized;

    fn ids(&self) -> Vec<i32>;

    fn state(&self) -> ProtocolState;

    fn priority(&self) -> ProtocolHandlerPriority {
        ProtocolHandlerPriority::SERVER
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection);
}

pub enum ProtocolHandlerPriority {
    EARLIEST,
    EARLY,
    SERVER,
    LATE,
    LATEST,
    MONITOR,
}

fn priority(priority: &ProtocolHandlerPriority) -> i32 {
    match priority {
        ProtocolHandlerPriority::EARLIEST => 0,
        ProtocolHandlerPriority::EARLY => 1,
        ProtocolHandlerPriority::SERVER => 2,
        ProtocolHandlerPriority::LATE => 3,
        ProtocolHandlerPriority::LATEST => 4,
        ProtocolHandlerPriority::MONITOR => 5,
    }
}

impl Eq for ProtocolHandlerPriority {}

impl PartialEq<Self> for ProtocolHandlerPriority {
    fn eq(&self, other: &Self) -> bool {
        priority(self) == priority(other)
    }
}

impl PartialOrd<Self> for ProtocolHandlerPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(priority(self).cmp(&priority(other)))
    }
}

impl Ord for ProtocolHandlerPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        priority(self).cmp(&priority(other))
    }
}
