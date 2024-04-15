use crate::define_packet;
use crate::network::connection::PlayerConnection;
use crate::packet::Packet;
use crate::packet::types::PacketStructure;
use crate::packet::types::string::MinecraftString;
use crate::protocol::{ProtocolHandler, ProtocolState};

pub(crate) struct StatusRequestHandler;

define_packet!(0x01, ServerboundStatusResponsePacket {
    data: MinecraftString
});

impl ProtocolHandler for StatusRequestHandler {
    fn new() -> Self {
        StatusRequestHandler
    }

    fn ids(&self) -> Vec<i32> {
        vec![0x00]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Status
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection) {
        println!("Received status request packet with id: {}", packet.id);
        let response = ServerboundStatusResponsePacket {
            data: MinecraftString::from(r#"{
            "version": {
                "name": "1.19.4",
                "protocol": 762
            },
            "players": {
                "max": 100,
                "online": 5,
                "sample": [
                    {
                        "name": "thinkofdeath",
                        "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                    }
                ]
            },
            "description": {
                "text": "Hello world"
            },
            "favicon": "data:image/png;base64,<data>",
            "enforcesSecureChat": true,
            "previewsChat": true
        }"#.to_string())
        };
        response.write(&mut connection.stream)
    }
}