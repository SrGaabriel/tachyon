use std::io::Write;
use std::net::TcpStream;
use crate::packet::Packet;

pub trait NetworkWriter {
    fn stream(&mut self) -> &mut TcpStream;

    fn write_packet(&mut self, packet: Packet);
}

pub struct UncompressedNetworkWriter {
    pub(crate) write: TcpStream
}

impl NetworkWriter for UncompressedNetworkWriter {
    fn stream(&mut self) -> &mut TcpStream {
        &mut self.write
    }

    fn write_packet(&mut self, packet: Packet) {
        // Write length of Packet ID + Data
        self.write.write(&packet.length.to_be_bytes()).unwrap();
        // Write ID of Packet
        self.write.write(&packet.id.to_be_bytes()).unwrap();
        // Write Data of Packet
        self.write.write_all(&packet.data.get_ref()).unwrap();
    }
}