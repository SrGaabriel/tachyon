use std::fs::File;
use std::io::{Read, Write};

use flate2::read::GzDecoder;

use crate::packet::ParsePacketError;
use crate::packet::types::PacketStructure;

impl PacketStructure for File {
    fn from_packet_data(buffer: &mut dyn Read) -> Result<Self, ParsePacketError> {
        panic!("File reading is not supported")
    }

    fn write_packet_data(&self, buffer: &mut dyn Write) {
        let mut _buffer = Vec::new();
        let mut decoder = GzDecoder::new(self);
        decoder.read_to_end(&mut _buffer).unwrap();
        buffer.write_all(&_buffer).unwrap();
    }
}