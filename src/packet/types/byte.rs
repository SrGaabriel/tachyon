use std::io::{Read, Write};

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for i8 {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut value = [0u8; 1];
        buffer.read_exact(&mut value).unwrap();
        Ok(value[0] as i8)
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        buffer.write(&[*self as u8]).unwrap();
    }
}

impl PacketStructure for u8 {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut value = [0u8; 1];
        buffer.read_exact(&mut value)?;
        Ok(value[0])
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        buffer.write(&[*self]).unwrap();
    }
}