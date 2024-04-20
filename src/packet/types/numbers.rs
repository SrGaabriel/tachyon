use std::io::{Read, Write};

use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftUnsignedShort {
    value: u16,
}

impl Into<u16> for MinecraftUnsignedShort {
    fn into(self) -> u16 {
        self.value
    }
}

impl From<u16> for MinecraftUnsignedShort {
    fn from(value: u16) -> Self {
        MinecraftUnsignedShort {
            value
        }
    }
}

impl PacketStructure<u16> for MinecraftUnsignedShort {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut bytes = [0; 2];
        buffer.read_exact(&mut bytes).unwrap();
        let value = u16::from_be_bytes(bytes);

        MinecraftUnsignedShort {
            value
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write_all(&self.value.to_be_bytes()).unwrap();
    }
}