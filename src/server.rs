use crate::network::TcpServer;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::ProtocolHandler;
use crate::protocol::status::StatusRequestHandler;
use crate::time::scheduler::{TaskScheduler};
use crate::time::tick::GameTimeManager;

pub struct TachyonServer {
    pub server_name: String,
    pub tcp_server: TcpServer,
    pub(crate) game_time_manager: GameTimeManager
}

impl TachyonServer {
    pub fn new(server_name: String, address: String) -> Self {
        TachyonServer {
            server_name,
            tcp_server: TcpServer::new(address),
            game_time_manager: GameTimeManager::new()
        }
    }

    pub fn start(&mut self) {
        let mut scheduler = TaskScheduler::new();
        self.game_time_manager.start(&scheduler);
        self.tcp_server.start(&mut scheduler);
        self.tcp_server.register_handler(Box::new(HandshakeRequestHandler::new()));
        self.tcp_server.register_handler(Box::new(StatusRequestHandler::new()));
    }
}