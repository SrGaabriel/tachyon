use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::network::connection::PlayerConnection;
use crate::network::writer::NetworkWriter;
use crate::packet::Packet;
use crate::packet::types::PacketStructure;
use crate::protocol::ProtocolHandler;
use crate::time::scheduler::TaskScheduler;

pub mod writer;
pub mod connection;

pub struct TcpServer {
    address: String,
    connections: HashMap<String, Arc<Mutex<PlayerConnection>>>,
    handlers: Vec<Box<dyn ProtocolHandler>>,
    packet_queue: Arc<Mutex<VecDeque<BoundPacket>>>
}

struct BoundPacket {
    packet: Packet,
    connection: Arc<Mutex<PlayerConnection>>
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

    pub fn call_handlers(&self, mut packet: Packet, connection: Arc<Mutex<PlayerConnection>>) {
        let mut connection = connection.lock().unwrap();
        let mut handlers = self.handlers.iter().map(|handler| handler.clone()).collect::<Vec<_>>();
        handlers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        handlers.iter().for_each(|handler| {
            if handler.state() == connection.state {
                handler.handle_packet(&mut packet, &mut connection);
            }
        });
        drop(connection);
    }

    pub fn start(&mut self, scheduler: &mut TaskScheduler) {
        let listener = std::net::TcpListener::bind(&self.address).unwrap();
        println!("Starting server on {}", self.address);

        scheduler.run_async_task(0, 1, move || {
            let mut packet_queue = self.packet_queue.lock().unwrap();
            while !packet_queue.is_empty() {
                let mut bound_packet = packet_queue.pop_front().unwrap();
                let mut connection = bound_packet.connection.lock().unwrap();
                bound_packet.packet.write(&mut connection.stream);
                drop(connection)
            }
        });

        let mut connections = self.connections.clone();  // Clone self.connections
        loop {
            let (stream, addr) = listener.accept().expect("Failed to accept connection");
            stream.set_nodelay(true).expect("Failed to set nodelay on stream");

            tokio::spawn(async move {
                let ip = stream.peer_addr().unwrap().to_string();
                let connection = PlayerConnection::new(
                    stream.try_clone().expect("Failed to clone stream")
                );
                let mutex = Arc::new(Mutex::new(connection));
                connections.insert(ip.clone(), mutex.clone());
                self.handle_client(mutex, stream);
            });
        }
    }

    fn handle_client(&self, connection: Arc<Mutex<PlayerConnection>>, mut stream: std::net::TcpStream) {
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();

            let packet = Packet::read(&mut &buffer[..bytes_read]);
            self.call_handlers(packet, Arc::clone(&connection));
        }
    }

    fn send_packet(&mut self, packet: Packet, connection: Arc<Mutex<PlayerConnection>>) {
        let bound_packet = BoundPacket {
            packet,
            connection
        };
        let mut queue = self.packet_queue.lock().unwrap();
        queue.push_back(bound_packet);
        drop(queue)
    }
}
