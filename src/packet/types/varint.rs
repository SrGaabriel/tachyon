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
        let mut value: u32 = 0;
        let mut position: u32 = 0;
        let mut current_byte: [u8; 1] = [0];

        loop {
            match buffer.read_exact(&mut current_byte) {
                Ok(_) => {
                    let current_byte = current_byte[0];
                    value |= (current_byte as u32 & SEGMENT_BITS as u32) << position;

                    if (current_byte as i32 & CONTINUE_BIT) == 0 {
                        break;
                    }

                    position += 7;

                    if position >= 32 {
                        panic!("VarInt is too big");
                    }
                }
                Err(e) => panic!("Failed to read VarInt: {}", e)
            }
        }
        MinecraftVarInt {
            value: value as i32
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
            size += 1;
            value >>= 7;

            if value == 0 {
                break;
            }
        }

        size
    }
}

#[derive(Debug)]
pub struct MinecraftInt {
    pub value: i32
}

impl Into<i32> for MinecraftInt {
    fn into(self) -> i32 {
        self.value
    }
}

impl From<i32> for MinecraftInt {
    fn from(value: i32) -> Self {
        MinecraftInt {
            value
        }
    }
}

impl PacketStructure<i32> for MinecraftInt {
    fn read(buffer: &mut dyn Read) -> Self {
        let mut value: [u8; 4] = [0; 4];
        buffer.read_exact(&mut value).unwrap();
        MinecraftInt {
            value: i32::from_be_bytes(value)
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write_all(&self.value.to_be_bytes()).unwrap();
    }
}