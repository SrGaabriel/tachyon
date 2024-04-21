#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, on_ground: bool) -> Self {
        Position {
            x,
            y,
            z,
            yaw: yaw.rem_euclid(360.0),
            pitch,
            on_ground
        }
    }

    pub fn from_vector_and_rotation(vector: Vector, rotation: Rotation, on_ground: bool) -> Self {
        Position {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            yaw: rotation.yaw,
            pitch: rotation.pitch,
            on_ground
        }
    }

    pub fn override_vector(vector: Vector) -> Self {
        Position {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            yaw: 270.0,
            pitch: 0.0,
            on_ground: false
        }
    }

    pub fn from_vector(vector: Vector, yaw: f32, pitch: f32, on_ground: bool) -> Self {
        Position {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            yaw: yaw.rem_euclid(360.0),
            pitch,
            on_ground
        }
    }

    pub fn as_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn as_rotation(&self) -> Rotation {
        Rotation {
            yaw: self.yaw,
            pitch: self.pitch
        }
    }

    pub fn distance(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            x,
            y,
            z
        }
    }

    pub fn calc_distance(one: &Vector, two: &Vector) -> f64 {
        ((one.x - two.x).powi(2) + (one.y - two.y).powi(2) + (one.z - two.z).powi(2)).sqrt()
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        Vector::calc_distance(self, other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Rotation {
            yaw,
            pitch
        }
    }

    pub fn from_look(standing_point: &Position, target_position: &Position) -> Rotation {
        let dx = target_position.x - standing_point.x;
        let dy = target_position.y - standing_point.y;
        let dz = target_position.z - standing_point.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        let yaw = (-dx.atan2(dz) / std::f64::consts::PI * 180.0).rem_euclid(360.0);
        let pitch = (-dy / distance).asin() / std::f64::consts::PI * 180.0;
        Rotation {
            yaw: yaw as f32,
            pitch: pitch as f32
        }
    }
}