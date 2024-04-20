use std::io::{Read, Write};

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for u16 {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut bytes = [0; 2];
        buffer.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        buffer.write_all(&self.to_be_bytes()).unwrap();
    }
}