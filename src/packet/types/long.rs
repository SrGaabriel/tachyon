use crate::packet::types::PacketStructure;

impl PacketStructure for i64 {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let mut data = [0; 8];
        buffer.read_exact(&mut data)?;
        Ok(i64::from_be_bytes(data))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        buffer.write_all(&self.to_be_bytes()).unwrap();
    }
}