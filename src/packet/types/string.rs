use std::io::{Read, Write};

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;

impl PacketStructure for String {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let length: i32 = VarInt::from_packet_data(buffer)?.into();
        let mut value = vec![0; length as usize];
        buffer.read_exact(&mut value)?;

        Ok(String::from_utf8(value)?)
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        let length = VarInt::from(self.len() as i32);
        length.write_packet_data(buffer);
        buffer.write_all(self.as_bytes()).unwrap();
    }
}