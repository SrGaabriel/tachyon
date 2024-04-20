use std::io::{Read, Write};
use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftByte {
    pub value: i8
}

impl Into<i8> for MinecraftByte {
    fn into(self) -> i8 {
        self.value
    }
}

impl From<i8> for MinecraftByte {
    fn from(value: i8) -> Self {
        MinecraftByte {
            value
        }
    }
}

impl PacketStructure<i8> for MinecraftByte {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut value = [0u8; 1];
        buffer.read_exact(&mut value).unwrap();
        MinecraftByte {
            value: value[0] as i8
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write(&[self.value as u8]).unwrap();
    }
}

#[derive(Debug)]
pub struct MinecraftUnsignedByte {
    pub value: u8
}

impl Into<u8> for MinecraftUnsignedByte {
    fn into(self) -> u8 {
        self.value
    }
}

impl From<u8> for MinecraftUnsignedByte {
    fn from(value: u8) -> Self {
        MinecraftUnsignedByte {
            value
        }
    }
}

impl PacketStructure<u8> for MinecraftUnsignedByte {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut value = [0u8; 1];
        buffer.read_exact(&mut value).unwrap();
        MinecraftUnsignedByte {
            value: value[0]
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write(&[self.value]).unwrap();
    }
}