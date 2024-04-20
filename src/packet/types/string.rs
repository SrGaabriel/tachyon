use std::fmt::Debug;
use std::io::{Read, Write};

use crate::packet::types::PacketStructure;
use crate::packet::types::varint::MinecraftVarInt;

#[derive(Debug)]
pub struct MinecraftString {
    pub value: String
}

impl Into<String> for MinecraftString {
    fn into(self) -> String {
        self.value
    }
}

impl From<String> for MinecraftString {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl PacketStructure<String> for MinecraftString {
    fn read(buffer: &mut dyn Read) -> Self {
        let length: i32 = MinecraftVarInt::read(buffer).into();
        let mut value = vec![0; length as usize];
        buffer.read_exact(&mut value).unwrap();

        let value = String::from_utf8(value).unwrap();
        MinecraftString {
            value
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        let length = MinecraftVarInt::from(self.value.len() as i32);
        length.write(buffer);
        buffer.write_all(self.value.as_bytes()).unwrap();
    }
}