use crate::game::math::position::{Position, Rotation, Vector};
use crate::packet::types::PacketStructure;

impl PacketStructure for Position {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let vector = Vector::from_packet_data(buffer)?;
        let rotation = Rotation::from_packet_data(buffer)?;
        let on_ground = bool::from_packet_data(buffer)?;
        Ok(Position::from_vector_and_rotation(vector, rotation, on_ground))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        self.as_vector().write_packet_data(buffer);
        self.as_rotation().write_packet_data(buffer);
        self.on_ground.write_packet_data(buffer);
    }
}

impl PacketStructure for Vector {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let x = f64::from_packet_data(buffer)?;
        let y = f64::from_packet_data(buffer)?;
        let z = f64::from_packet_data(buffer)?;
        Ok(Vector::new(x, y, z))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        self.x.write_packet_data(buffer);
        self.y.write_packet_data(buffer);
        self.z.write_packet_data(buffer);
    }
}

impl PacketStructure for Rotation {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let yaw = f32::from_packet_data(buffer)?;
        let pitch = f32::from_packet_data(buffer)?;
        Ok(Rotation::new(yaw, pitch))
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        self.yaw.write_packet_data(buffer);
        self.pitch.write_packet_data(buffer);
    }
}

#[derive(Debug)]
pub struct EncodedVector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl EncodedVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        EncodedVector {
            x,
            y,
            z
        }
    }

    pub fn from_vector(vector: Vector) -> Self {
        EncodedVector {
            x: vector.x,
            y: vector.y,
            z: vector.z
        }
    }

    pub fn as_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl PacketStructure for EncodedVector {
    fn from_packet_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
        let long = i64::from_packet_data(buffer)?;
        let x = (long >> 38);
        let y = ((long << 52) >> 52);
        let z = ((long << 26) >> 38);
        Ok(EncodedVector {
            x: if x >= 1 << 25 { x - (1 << 26) } else { x } as f64,
            y: if y >= 1 << 11 { y - (1 << 12) } else { y } as f64,
            z: if z >= 1 << 25 { z - (1 << 26) } else { z } as f64,
        })
    }

    fn write_packet_data(&self, buffer: &mut dyn std::io::Write) {
        let long = (((self.x as i64) & 0x3FFFFFF) << 38) | (((self.z as i64) & 0x3FFFFFF) << 12)
            | ((self.y as i64) & 0xFFF);
        long.write_packet_data(buffer);
    }
}