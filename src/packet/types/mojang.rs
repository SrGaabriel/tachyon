use crate::mojang::SkinProperty;
use crate::packet::types::PacketStructure;

impl PacketStructure for SkinProperty {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let name = String::from_packet_data(buffer)?;
        let value = String::from_packet_data(buffer)?;
        let has_signature = bool::from_packet_data(buffer)?;
        let signature = if has_signature {
            Some(String::from_packet_data(buffer)?)
        } else {
            None
        };
        Ok(SkinProperty {
            name,
            value,
            signature
        })
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        self.name.write_packet_data(buffer);
        self.value.write_packet_data(buffer);
        self.signature.is_some().write_packet_data(buffer);
        if let Some(signature) = &self.signature {
            signature.write_packet_data(buffer);
        }
    }
}