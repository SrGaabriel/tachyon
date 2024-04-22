use std::io::{Cursor, Read, Write};
use flate2::read::{ZlibDecoder, ZlibEncoder};

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

    pub fn parse(buffer: &mut dyn Read, compression_threshold: Option<i32>) -> Result<Self, ParsePacketError> {
        if compression_threshold.is_none() || compression_threshold.unwrap() < 0 {
            return Packet::parse_uncompressed(buffer);
        }
        let packet_length: i32 = VarInt::from_packet_data(buffer)?.into();
        let data_length = VarInt::from_packet_data(buffer)?;
        let mut decoder = ZlibDecoder::new(buffer);
        let mut compressed_data = vec![0; data_length.value as usize];
        decoder.read_exact(&mut compressed_data)?;
        let mut compressed_data = Cursor::new(compressed_data);
        let packet_id: i32 = VarInt::from_packet_data(&mut compressed_data)?.into();

        Ok(Packet {
            length: packet_length as usize,
            id: packet_id.into(),
            data: compressed_data
        })
    }

    pub fn parse_uncompressed(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
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

    pub fn write(&self, buffer: &mut dyn Write, compression_threshold: Option<i32>) {
        if compression_threshold.is_none() || compression_threshold.unwrap() < 0 || self.data.get_ref().len() < compression_threshold.unwrap() as usize {
            self.write_uncompressed(buffer);
            return;
        }
        let encoder_buffer = Cursor::new(Vec::new());
        let mut encoder = ZlibEncoder::new(
            encoder_buffer,
            flate2::Compression::default()
        );
        encoder.write_all(&self.id.to_be_bytes()).expect("Failed to write id to encoder");
        encoder.write_all(&self.data.get_ref()).expect("Failed to write data to encoder");
        let mut compressed_data: Vec<u8> = vec![];
        encoder.read_to_end(&mut compressed_data).expect("Failed to read compressed data from encoder");
        if compressed_data.len() < compression_threshold.unwrap() as usize {
            self.write_uncompressed(buffer);
            return;
        }
        let compressed_length = compressed_data.len();
        let packet_length = VarInt::from((compressed_length + VarInt::size(compressed_length as i32)) as i32);
        packet_length.write_packet_data(buffer);
        VarInt::from(compressed_length as i32).write_packet_data(buffer);
        buffer.write_all(&compressed_data).expect("Failed to write compressed data to buffer");
    }

    pub fn write_uncompressed(&self, buffer: &mut dyn Write) {
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