use std::collections::HashMap;
use std::net::SocketAddr;
use uuid::Uuid;
use crate::game::MessageType;
use crate::game::entity::Player;
use crate::game::text::TextComponent;
use crate::network::TcpServer;
use crate::packet::Packet;
use crate::protocol;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::login::LoginProtocolHandler;
use crate::protocol::status::StatusRequestHandler;
use crate::security::Keypair;

pub struct TachyonServer {
    pub(crate) keypair: Keypair,
    pub players: HashMap<Uuid, Box<dyn Player>>
}

impl TachyonServer {
    pub(crate) fn start(&mut self) {
        let mut tcp_server = TcpServer::new(SocketAddr::from(([127, 0, 0, 1], 25565)));
        tcp_server.register_handler(Box::new(HandshakeRequestHandler {}));
        tcp_server.register_handler(Box::new(StatusRequestHandler {}));
        tcp_server.register_handler(Box::new(LoginProtocolHandler { encrypted: true }));
        protocol::play::register_all_handlers(&mut tcp_server);
        tcp_server.start(self);
    }

    pub fn broadcast_message(&mut self, message: TextComponent, chat: MessageType) {
        for player in self.players.values_mut() {
            if player.get_chat_mode().accepts_chat(chat) {
                player.send_message(message.clone());
            }
        }
    }

    pub(crate) fn broadcast_packet(&mut self, packet: &mut Packet) {
        for player in self.players.values_mut() {
            player.send_packet(packet);
        }
    }
}