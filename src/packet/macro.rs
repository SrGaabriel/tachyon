#[macro_export]
macro_rules! define_packet {
    ($id:expr, $struct_name:ident {
        $($field:ident : $field_type:ty),*
    }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $field: $field_type),*
        }

        impl crate::packet::PacketDefinition for $struct_name {
            fn get_id() -> i32 {
                $id
            }

            fn write_data(&self, buffer: &mut dyn std::io::Write) {
                $(self.$field.write_packet_data(buffer);)*
            }

            fn read_data(buffer: &mut dyn std::io::Read) -> Result<Self, crate::packet::ParsePacketError> {
                Ok($struct_name {
                    $($field: <$field_type as PacketStructure>::from_packet_data(buffer)?),*
                })
            }

            fn to_packet(&self) -> Packet {
                let mut data = Vec::new();
                self.write_data(&mut data);
                Packet::new($id, data)
            }
        }
    };
}