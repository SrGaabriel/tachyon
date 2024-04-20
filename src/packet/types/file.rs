use std::fs::File;
use std::io::{Read, Write};
use flate2::read::GzDecoder;
use crate::packet::types::PacketStructure;

#[derive(Debug)]
pub struct MinecraftFile {
    pub file: File,
}

impl Into<File> for MinecraftFile {
    fn into(self) -> File {
        self.file
    }
}

impl From<File> for MinecraftFile {
    fn from(value: File) -> Self {
        MinecraftFile {
            file: value,
        }
    }
}

impl PacketStructure<File> for MinecraftFile {
    fn read(buffer: &mut dyn Read) -> Self {
        panic!("File reading is not supported")
    }

    fn write(&self, buffer: &mut dyn Write) {
        let mut _buffer = Vec::new();
        let mut decoder = GzDecoder::new(&self.file);
        decoder.read_to_end(&mut _buffer).unwrap();
        buffer.write_all(&_buffer).unwrap();
    }
}