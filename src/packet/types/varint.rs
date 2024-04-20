use std::io::{Read, Write};

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

pub const SEGMENT_BITS: i32 = 0x7F;
pub const CONTINUE_BIT: i32 = 0x80;

#[derive(Debug)]
pub struct VarInt {
    pub value: i32
}

impl PacketStructure for VarInt {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut value: u32 = 0;
        let mut position: u32 = 0;
        let mut current_byte: [u8; 1] = [0];

        loop {
            buffer.read_exact(&mut current_byte)?;
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
        Ok(VarInt { value: value as i32 })
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
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

impl VarInt {
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

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.value
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt {
            value
        }
    }
}

impl PacketStructure for i32 {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let mut value: [u8; 4] = [0; 4];
        buffer.read_exact(&mut value).unwrap();
        Ok(i32::from_be_bytes(value))
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        buffer.write_all(&self.to_be_bytes()).unwrap();
    }
}