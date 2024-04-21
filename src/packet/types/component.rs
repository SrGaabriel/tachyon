use crate::game::text::TextComponent;
use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct JsonTextComponent {
    pub value: TextComponent
}

impl Into<JsonTextComponent> for TextComponent {
    fn into(self) -> JsonTextComponent {
        JsonTextComponent {
            value: self
        }
    }
}

#[derive(Debug)]
pub struct NbtTextComponent {
    pub value: TextComponent
}

impl PacketStructure for JsonTextComponent {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, ParsePacketError> {
        let value = String::from_packet_data(buffer)?;
        let serialization: TextComponent = serde_json::from_str(&value)
            .map_err(|e| ParsePacketError::new(format!("Failed to parse json text component: {}", e)))?;
        Ok(JsonTextComponent {
            value: serialization
        })
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        let value = serde_json::to_string(&self.value)
            .expect("Failed to serialize json text component");
        value.write_packet_data(buffer);
    }
}

impl PacketStructure for NbtTextComponent {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, ParsePacketError> {
        let nbt: TextComponent = fastnbt::from_reader(buffer)
            .map_err(|e| ParsePacketError::new(format!("Failed to parse nbt text component: {}", e)))?;
        Ok(NbtTextComponent {
            value: nbt
        })
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        fastnbt::to_writer(buffer, &self.value).unwrap();
    }
}