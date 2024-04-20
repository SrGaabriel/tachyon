use std::io::{Read, Write};

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

pub trait PacketStructure<T>: Into<T> + From<T> {
    fn read(buffer: &mut dyn Read) -> Self;

    fn write(&self, buffer: &mut dyn Write);
}