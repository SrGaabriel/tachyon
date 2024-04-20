use std::io::{Read, Write};
use uuid::Uuid;
use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftUuid {
    pub value: Uuid
}

impl Into<Uuid> for MinecraftUuid {
    fn into(self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for MinecraftUuid {
    fn from(value: Uuid) -> Self {
        MinecraftUuid {
            value
        }
    }
}

// Encoded as an unsigned 128-bit integer (or two unsigned 64-bit integers: the most significant 64 bits and then the least significant 64 bits)
impl PacketStructure<Uuid> for MinecraftUuid {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut uuid_buffer = [0u8; 16];
        buffer.read_exact(&mut uuid_buffer).unwrap();
        let value = Uuid::from_bytes(uuid_buffer);
        return MinecraftUuid {
            value
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write_all(self.value.as_bytes()).unwrap();
    }
}