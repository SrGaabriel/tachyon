use std::io::{Read, Write};

use crate::packet::ParsePacketError;

pub mod varint;
pub mod string;
pub mod numbers;
pub mod uuid;
pub mod vec;
pub mod boolean;
pub mod byte;
pub mod nbt;
pub mod file;
pub mod identifier;

pub trait PacketStructure where Self: Sized {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError>;

    fn write_packet_data(&self, buffer: &mut dyn Write);
}