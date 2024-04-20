use std::io::{Read, Write};

use uuid::Uuid;

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for Uuid {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut bytes = [0; 16];
        buffer.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        buffer.write_all(self.as_bytes()).unwrap();
    }
}