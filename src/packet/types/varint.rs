use std::io::{Read, Write};

use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftVarInt {
    pub value: i32
}

impl Into<i32> for MinecraftVarInt {
    fn into(self) -> i32 {
        self.value
    }
}

impl From<i32> for MinecraftVarInt {
    fn from(value: i32) -> Self {
        MinecraftVarInt {
            value
        }
    }
}

pub const SEGMENT_BITS: i32 = 0x7F;
pub const CONTINUE_BIT: i32 = 0x80;

impl PacketStructure<i32> for MinecraftVarInt {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut value = 0;
        let mut pos = 0;
        let mut byte = [0; 1];

        loop {
            buffer.read_exact(&mut byte).unwrap();
            let byte = byte[0] as i32;

            value |= (byte & SEGMENT_BITS) << pos;

            if byte & CONTINUE_BIT == 0 {
                break;
            }

            pos += 7;

            if pos > 32 {
                panic!("VarInt is too big");
            }
        }

        MinecraftVarInt {
            value
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        let mut value = self.value;

        loop {
            if (value & !SEGMENT_BITS) == 0 {
                buffer.write(&[value as u8]).unwrap();
                break;
            }

            buffer.write_all(&[(value & SEGMENT_BITS | CONTINUE_BIT) as u8]).unwrap();
            value >>= 7;
        }
    }
}

impl MinecraftVarInt {
    pub fn size(value: i32) -> usize {
        let mut value = value;
        let mut size = 0;
        loop {
            let byte = (value & SEGMENT_BITS) as u8;
            value >>= 7;
            size += 1;
            if value == 0 {
                break;
            }
        }
        size
    }
}
