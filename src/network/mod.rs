use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::network::connection::PlayerConnection;
use crate::network::writer::NetworkWriter;
use crate::packet::Packet;
use crate::packet::types::PacketStructure;
use crate::protocol::ProtocolHandler;
use crate::server::TachyonServer;

pub mod writer;
pub mod connection;

pub struct TcpServer {
    address: String,
    connections: HashMap<String, Rc<RefCell<PlayerConnection>>>,
    handlers: Vec<Box<dyn ProtocolHandler>>,
    packet_queue: Arc<Mutex<VecDeque<BoundPacket>>>
}

struct BoundPacket {
    packet: Packet,
    connection: Rc<RefCell<PlayerConnection>>
}

impl TcpServer {
    pub fn new(address: String) -> Self {
        TcpServer {
            address,
            connections: HashMap::new(),
            handlers: Vec::new(),
            packet_queue: Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn ProtocolHandler>) {
        self.handlers.push(handler);
    }

    pub fn call_handlers(&self, mut packet: Packet, connection: Rc<RefCell<PlayerConnection>>) {
        let mut handlers = self.handlers.iter().map(|handler| handler.clone()).collect::<Vec<_>>();
        handlers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        handlers.iter().for_each(|handler| {
            if handler.state() == connection.borrow().state {
                handler.handle_packet(&mut packet, &mut connection.borrow_mut());
            }
        });
    }

    pub fn start(&mut self, server: &mut TachyonServer) {
        let listener = std::net::TcpListener::bind(&self.address).unwrap();
        println!("Starting server on {}", self.address);

        server.scheduler.run_async_task(0, 1, move || {
            let mut packet_queue = server.tcp_server.packet_queue.lock().unwrap();
            while !packet_queue.is_empty() {
                let bound_packet = packet_queue.pop_front().unwrap();
                bound_packet.packet.write(&mut bound_packet.connection.borrow_mut().stream);
            }
        });

        let mut connections = self.connections.clone();  // Clone self.connections
        loop {
            let (stream, addr) = listener.accept().expect("Failed to accept connection");
            stream.set_nodelay(true).expect("Failed to set nodelay on stream");

            tokio::spawn(async move {
                let ip = stream.peer_addr().unwrap().to_string();
                let connection = connections.entry(ip.clone()).or_insert_with(|| {
                    Rc::new(RefCell::new(PlayerConnection::new(stream.try_clone().unwrap())))
                });
                connection.borrow_mut().stream = stream.try_clone().unwrap();
                self.handle_client(Rc::clone(connection), stream);
            });
        }
    }

    fn handle_client(&self, connection: Rc<RefCell<PlayerConnection>>, mut stream: std::net::TcpStream) {
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();

            let packet = Packet::read(&mut &buffer[..bytes_read]);
            self.call_handlers(packet, Rc::clone(&connection));
        }
    }

    fn send_packet(&mut self, packet: Packet, connection: Rc<RefCell<PlayerConnection>>) {
        let bound_packet = BoundPacket {
            packet,
            connection
        };
        self.packet_queue.push_back(bound_packet);
    }
}
