use std::io::{Cursor, Read, Write};

use crate::packet::types::PacketStructure;
use crate::packet::types::varint::VarInt;

pub mod types;
mod r#macro;

#[derive(Debug)]
pub struct ParsePacketError {
    pub message: String
}

impl From<std::io::Error> for ParsePacketError {
    fn from(error: std::io::Error) -> Self {
        ParsePacketError {
            message: error.to_string()
        }
    }
}

impl From<std::string::FromUtf8Error> for ParsePacketError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ParsePacketError {
            message: error.to_string()
        }
    }
}

impl ParsePacketError {
    pub fn new(message: String) -> Self {
        ParsePacketError {
            message
        }
    }
}

#[derive(Clone)]
pub struct Packet {
    pub length: usize,
    pub id: i32,
    pub data: Cursor<Vec<u8>>
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        let length = VarInt::size(id) + data.len();
        let data = Cursor::new(data);

        Packet {
            length,
            id,
            data
        }
    }

    pub fn parse(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        let length = <VarInt as Into<i32>>::into(VarInt::from_packet_data(buffer)?) as usize;
        let id = VarInt::from_packet_data(buffer)?.into();

        let mut data = vec![0; length - VarInt::size(id)];
        buffer.read_exact(&mut data)?;

        Ok(Packet {
            length,
            id,
            data: Cursor::new(data)
        })
    }

    pub fn write(&self, buffer: &mut dyn Write) {
        VarInt::from(self.length as i32).write_packet_data(buffer);
        VarInt::from(self.id).write_packet_data(buffer);
        buffer.write_all(&self.data.get_ref()).unwrap();
    }
}

pub trait PacketDefinition {
    fn get_id() -> i32;

    fn write_data(&self, buffer: &mut dyn Write) {
        panic!("Packet {} does not implement write_data", Self::get_id());
    }

    fn read_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> where Self: Sized {
        panic!("Packet {} does not implement read_data", Self::get_id());
    }

    fn to_packet(&self) -> Packet {
        let mut data = Vec::new();
        self.write_data(&mut data);
        Packet::new(Self::get_id(), data)
    }
}