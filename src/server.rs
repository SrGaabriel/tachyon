use crate::network::TcpServer;
use crate::protocol::handshake::HandshakeRequestHandler;
use crate::protocol::ProtocolHandler;
use crate::protocol::status::StatusRequestHandler;
use crate::time::scheduler::TaskScheduler;
use crate::time::tick::GameTimeManager;

pub struct TachyonServer {
    pub server_name: String,
    pub tcp_server: TcpServer,
    pub scheduler: Box<dyn TaskScheduler>,
    pub(crate) game_time_manager: GameTimeManager
}

impl TachyonServer {
    pub fn new(server_name: String, address: String) -> Self {
        TachyonServer {
            server_name,
            tcp_server: TcpServer::new(address),
            scheduler: Box::new(TaskScheduler::new()),
            game_time_manager: GameTimeManager::new()
        }
    }

    pub fn start(&mut self) {
        self.game_time_manager.start(self);
        self.tcp_server.start(self);
        self.tcp_server.register_handler(Box::new(HandshakeRequestHandler::new()));
        self.tcp_server.register_handler(Box::new(StatusRequestHandler::new()));
    }
}