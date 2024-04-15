pub mod types;
mod r#macro;

use std::io::{Cursor, Read, Write};
use crate::packet::types::PacketStructure;
use crate::packet::types::varint::MinecraftVarInt;

pub struct Packet {
    pub length: i32,
    pub id: i32,
    pub data: Cursor<Vec<u8>>
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        let length = std::mem::size_of::<i32>() as i32 + data.len() as i32;
        let data = Cursor::new(data);

        Packet {
            length,
            id,
            data
        }
    }
}

impl PacketStructure<Packet> for Packet {
    fn read(buffer: &mut dyn Read) -> Self {
        let length: i32 = MinecraftVarInt::read(buffer).into();
        let id: i32 = MinecraftVarInt::read(buffer).into();

        let mut data = vec![0; length as usize - MinecraftVarInt::size(id)];
        println!("Reading packet with length: {}", length);
        println!("Reading packet with id: {}", id);
        println!("Reading packet with calculated size: {}", length as usize - MinecraftVarInt::size(id));
        println!("Reading packet with data size: {:?}", data.len());
        buffer.read_exact(&mut data).unwrap();

        Packet {
            length,
            id,
            data: Cursor::new(data)
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        buffer.write(&self.length.to_be_bytes()).unwrap();
        buffer.write(&self.id.to_be_bytes()).unwrap();
        buffer.write_all(&self.data.get_ref()).unwrap();
    }
}