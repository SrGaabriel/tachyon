#[macro_export]
macro_rules! define_packet {
    ($id:expr, $struct_name:ident {
        $($field:ident : $field_type:ty),*
    }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $field: $field_type),*
        }

        impl crate::packet::types::PacketStructure<$struct_name> for $struct_name {
            fn read(buffer: &mut dyn std::io::Read) -> Self {
                $struct_name {
                    $($field: <$field_type>::read(buffer)),*
                }
            }

            fn write(&self, buffer: &mut dyn std::io::Write) {
                $(self.$field.write(buffer));*
            }
        }
    };
}