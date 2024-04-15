use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::rc::Rc;

use crate::network::connection::PlayerConnection;
use crate::network::writer::NetworkWriter;
use crate::packet::Packet;
use crate::packet::types::PacketStructure;
use crate::protocol::ProtocolHandler;

pub mod writer;
pub mod connection;

pub struct TcpServer {
    address: String,
    connections: HashMap<String, Rc<RefCell<PlayerConnection>>>,
    handlers: Vec<Box<dyn ProtocolHandler>>,
}

impl TcpServer {
    pub fn new(address: String) -> Self {
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
        let mut handlers = self.handlers.iter().map(|handler| handler.clone()).collect::<Vec<_>>();
        handlers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        handlers.iter().for_each(|handler| {
            if handler.state() == connection.borrow().state {
                handler.handle_packet(&mut packet, &mut connection.borrow_mut());
            }
        });
    }

    pub fn start(&mut self) {
        let listener = std::net::TcpListener::bind(&self.address).unwrap();
        println!("Starting server on {}", self.address);

        let mut connections = self.connections.clone();  // Clone self.connections

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let ip = stream.peer_addr().unwrap().to_string();
            let connection = connections.entry(ip.clone()).or_insert_with(|| {
                Rc::new(RefCell::new(PlayerConnection::new(stream.try_clone().unwrap())))
            });
            connection.borrow_mut().stream = stream.try_clone().unwrap();
            self.handle_client(Rc::clone(connection), stream);
            // if connection_option.is_none() {
            //     let connection = Rc::new(RefCell::new(PlayerConnection::new(stream.try_clone().unwrap())));
            //     connections.insert(ip.clone(), Rc::clone(&connection));
            //     self.handle_client(Rc::clone(&connection), stream);
            // } else {
            //     let connection = connection_option.unwrap();
            //     connection.borrow_mut().stream = stream.try_clone().unwrap();
            //     self.handle_client(Rc::clone(connection), stream);
            // }
        }
    }

    fn handle_client(&self, connection: Rc<RefCell<PlayerConnection>>, mut stream: std::net::TcpStream) {
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }

            let packet = Packet::read(&mut &buffer[..bytes_read]);
            self.call_handlers(packet, Rc::clone(&connection));
        }
    }
}
