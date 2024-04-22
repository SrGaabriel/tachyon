use crate::game::text::TextComponent;
use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for TextComponent {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, ParsePacketError> {
        let value = String::from_packet_data(buffer)?;
        serde_json::from_str(&value)
            .map_err(|e| ParsePacketError::new(format!("Failed to parse json text component: {}", e)))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        let value = serde_json::to_string(self)
            .expect("Failed to serialize json text component");
        value.write_packet_data(buffer);
    }
}
