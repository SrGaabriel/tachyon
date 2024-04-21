use crate::packet::types::PacketStructure;

impl PacketStructure for f32 {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let mut data = [0; 4];
        buffer.read_exact(&mut data)?;
        Ok(Self::from_be_bytes(data))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        buffer.write_all(&self.to_le_bytes()).unwrap();
    }
}

impl PacketStructure for f64 {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let mut data = [0; 8];
        buffer.read_exact(&mut data)?;
        Ok(Self::from_be_bytes(data))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        buffer.write_all(&self.to_le_bytes()).unwrap();
    }
}