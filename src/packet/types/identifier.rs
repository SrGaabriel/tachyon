use std::io::{Read, Write};
use crate::game::identifier::NamespaceId;
use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for NamespaceId {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let text = String::from_packet_data(buffer)?;
        Ok(NamespaceId::from_string(text))
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        self.to_string().write_packet_data(buffer);
    }
}