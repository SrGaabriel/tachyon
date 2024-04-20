use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::net::SocketAddr;
use std::rc::Rc;

use crate::network::connection::PlayerConnection;
use crate::packet::Packet;
use crate::protocol::ProtocolHandler;

pub mod connection;

pub struct TcpServer {
    address: SocketAddr,
    connections: HashMap<SocketAddr, Rc<RefCell<PlayerConnection>>>,
    handlers: Vec<Box<dyn ProtocolHandler>>,
}

impl TcpServer {
    pub fn new(address: SocketAddr) -> Self {
        TcpServer {
            address,
            connections: HashMap::new(),
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn ProtocolHandler>) {
        self.handlers.push(handler);
    }

    pub fn call_handlers(&self, mut packet: Packet, connection: Rc<RefCell<PlayerConnection>>) {
        let state = connection.borrow().state;
        let mut handlers = self.handlers.iter().collect::<Vec<_>>();
        handlers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        handlers.iter().for_each(|handler| {
            if handler.state() == state && handler.ids().contains(&packet.id) {
                handler.handle_packet(&mut packet, &mut connection.borrow_mut());
            }
        });
    }

    pub fn start(&mut self) {
        let listener = std::net::TcpListener::bind(self.address).unwrap();
        println!("Starting server on {}", self.address);

        loop {
            let (stream, peer) = listener.accept().unwrap();
            stream.set_nodelay(false).expect("Failed to set nodelay on stream");
            let connection = Rc::new(RefCell::new(PlayerConnection::new(stream.try_clone().unwrap())));
            self.connections.insert(peer, Rc::clone(&connection));
            self.handle_client(Rc::clone(&connection), stream);
        }
    }

    fn handle_client(&self, connection: Rc<RefCell<PlayerConnection>>, mut stream: std::net::TcpStream) {
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }

            let packet = Packet::parse(&mut &buffer[..bytes_read]).expect("Failed to parse packet");
            println!("Received packet with id: {}", packet.id);
            self.call_handlers(packet, Rc::clone(&connection));
        }
    }
}
