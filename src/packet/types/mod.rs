pub mod varint;
pub mod string;
pub mod numbers;

use std::io::{Read, Write};

pub trait PacketStructure<T>: Into<T> + From<T> {
    fn read(buffer: &mut dyn Read) -> Self;

    fn write(&self, buffer: &mut dyn Write);
}