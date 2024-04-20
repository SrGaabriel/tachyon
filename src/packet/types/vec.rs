use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;

impl<T> PacketStructure for Vec<T> where T: PacketStructure + Clone{
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, ParsePacketError> {
        let length: i32 = VarInt::from_packet_data(buffer)?.into();
        let mut value = Vec::new();
        for _ in 0..length {
            value.push(T::from_packet_data(buffer)?.into());
        }
        Ok(value)
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        let length = VarInt::from(self.len() as i32);
        length.write_packet_data(buffer);
        for item in self {
            T::from(item.clone()).write_packet_data(buffer)
        }
    }
}