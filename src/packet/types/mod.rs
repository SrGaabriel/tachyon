use std::io::{Read, Write};

use crate::packet::ParsePacketError;

pub mod varint;
pub mod string;
pub mod numbers;
pub mod uuid;
pub mod vec;
pub mod boolean;
pub mod byte;
pub mod component;
pub mod file;
pub mod identifier;
pub mod long;
pub mod enums;
pub mod position;
pub mod float;
pub mod mojang;

pub trait PacketStructure where Self: Sized {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError>;

    fn write_packet_data(&self, buffer: &mut dyn Write);
}