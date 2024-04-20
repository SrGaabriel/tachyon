use std::io::{Cursor, Read, Write};

use crate::packet::types::PacketStructure;
use crate::packet::types::varint::MinecraftVarInt;

pub mod types;
mod r#macro;

#[derive(Clone)]
pub struct Packet {
    pub length: usize,
    pub id: i32,
    pub data: Cursor<Vec<u8>>
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        let length = MinecraftVarInt::size(id) + data.len();
        let data = Cursor::new(data);

        Packet {
            length,
            id,
            data
        }
    }

    pub fn as_struct<T: PacketStructure<T>>(&self) -> T {
        T::read(&mut self.data.clone())
    }
}

impl PacketStructure<Packet> for Packet {
    fn read(buffer: &mut dyn Read) -> Self {
        let length = <MinecraftVarInt as Into<i32>>::into(MinecraftVarInt::read(buffer)) as usize;
        let id = MinecraftVarInt::read(buffer).into();

        let mut data = vec![0; length - MinecraftVarInt::size(id)];
        buffer.read_exact(&mut data).unwrap();

        Packet {
            length,
            id,
            data: Cursor::new(data)
        }
    }

    fn write(&self, buffer: &mut dyn Write) {
        MinecraftVarInt::from(self.length as i32).write(buffer);
        MinecraftVarInt::from(self.id).write(buffer);
        buffer.write_all(&self.data.get_ref()).unwrap();
    }
}

pub trait PacketDefinition {
    fn get_id() -> i32;

    fn write_data(&self, buffer: &mut dyn Write) {
        panic!("Packet {} does not implement write_data", Self::get_id());
    }

    fn read_data(buffer: &mut dyn Read) -> Self where Self: Sized {
        panic!("Packet {} does not implement read_data", Self::get_id());
    }

    fn to_packet(&self) -> Packet {
        let mut data = Vec::new();
        self.write_data(&mut data);
        Packet::new(Self::get_id(), data)
    }
}