use crate::define_packet;
use crate::network::connection::PlayerConnection;
use crate::packet::{Packet, PacketDefinition};
use crate::packet::types::PacketStructure;
use crate::protocol::{ProtocolHandler, ProtocolState};

pub(crate) struct StatusRequestHandler;

define_packet!(0x00, ClientboundStatusPacket {
    data: String
});

impl ProtocolHandler for StatusRequestHandler {
    fn new() -> Self {
        StatusRequestHandler
    }

    fn ids(&self) -> Vec<i32> {
        vec![0x00, 0x01]
    }

    fn state(&self) -> ProtocolState {
        ProtocolState::Status
    }

    fn handle_packet(&self, packet: &mut Packet, connection: &mut PlayerConnection) {
        if packet.id == 0x01 {
            connection.dispatch(packet);
            connection.close_gracefully();
            return;
        }

        let response = ClientboundStatusPacket {
            data: r#"{
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
        }"#.to_string()
        };
        // Send packet id, length, and data
        println!("Sending status response packet with id: {}", response.to_packet().id);
        connection.dispatch(&mut response.to_packet());
    }
}