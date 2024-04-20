use crate::packet::types::PacketStructure;
use crate::packet::types::varint::MinecraftVarInt;

#[derive(Debug)]
pub struct MinecraftVec<O : Clone, T : PacketStructure<O>> {
    pub value: Vec<O>,
    // mark the type as a PhantomData
    phantom: std::marker::PhantomData<T>
}

impl<O : Clone, T : PacketStructure<O>> Into<Vec<O>> for MinecraftVec<O, T> {
    fn into(self) -> Vec<O> {
        self.value
    }
}

impl<O : Clone, T : PacketStructure<O>> From<Vec<O>> for MinecraftVec<O, T> {
    fn from(value: Vec<O>) -> Self {
        Self { value, phantom: std::marker::PhantomData }
    }
}

impl<O : Clone, T : PacketStructure<O>> PacketStructure<Vec<O>> for MinecraftVec<O, T> {
    fn read(buffer: &mut dyn std::io::Read) -> Self {
        let length: i32 = MinecraftVarInt::read(buffer).into();
        let mut value = Vec::new();
        for _ in 0..length {
            value.push(T::read(buffer).into());
        }

        MinecraftVec {
            value,
            phantom: std::marker::PhantomData
        }
    }

    fn write(&self, buffer: &mut dyn std::io::Write) {
        let length = MinecraftVarInt::from(self.value.len() as i32);
        length.write(buffer);
        for item in &self.value {
            T::from(item.clone()).write(buffer)
        }
    }
}