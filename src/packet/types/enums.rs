use std::io::{Read, Write};
use crate::game::{ChatMode, Gamemode};
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

impl PacketStructure for Gamemode {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let varint = VarInt::from_packet_data(buffer)?;
        match varint.value {
            0 => Ok(Gamemode::Survival),
            1 => Ok(Gamemode::Creative),
            2 => Ok(Gamemode::Adventure),
            3 => Ok(Gamemode::Spectator),
            _ => Err(ParsePacketError::new(format!("Invalid gamemode: {}", varint.value)))
        }
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        let varint = match self {
            Gamemode::Survival => VarInt::from(0),
            Gamemode::Creative => VarInt::from(1),
            Gamemode::Adventure => VarInt::from(2),
            Gamemode::Spectator => VarInt::from(3)
        };
        varint.write_packet_data(buffer);
    }
}