use std::io::{Read, Write};
use crate::game::ChatMode;
use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;

impl PacketStructure for ChatMode {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let varint = VarInt::from_packet_data(buffer)?;
        match varint.value {
            0 => Ok(ChatMode::Full),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(ParsePacketError::new(format!("Invalid chat mode: {}", varint.value)))
        }
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        let varint = match self {
            ChatMode::Full => VarInt::from(0),
            ChatMode::CommandsOnly => VarInt::from(1),
            ChatMode::Hidden => VarInt::from(2)
        };
        varint.write_packet_data(buffer);
    }
}