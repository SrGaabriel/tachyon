use std::io::{Read, Write};

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for bool {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let byte = u8::from_packet_data(buffer)?;
        Ok(byte != 0)
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        (*self as u8).write_packet_data(buffer);
    }
}