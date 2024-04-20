use std::io::{Read, Write};
use crate::packet::types::byte::MinecraftByte;
use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftBoolean {
    pub value: bool
}

impl Into<bool> for MinecraftBoolean {
    fn into(self) -> bool {
        self.value
    }
}

impl From<bool> for MinecraftBoolean {
    fn from(value: bool) -> Self {
        MinecraftBoolean {
            value
        }
    }
}

impl PacketStructure<bool> for MinecraftBoolean {
    fn read(buffer: &mut dyn Read) -> Self {
        let byte = MinecraftByte::read(buffer);
        MinecraftBoolean {
            value: byte.value == 1
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        let byte = if self.value {
            MinecraftByte::from(1)
        } else {
            MinecraftByte::from(0)
        };
        byte.write(buffer);
    }
}