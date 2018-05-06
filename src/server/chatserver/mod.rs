extern crate websocket;
extern crate serde;
extern crate serde_json;

use std::collections::BTreeSet;
use std::net::{SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use websocket::server::{NoTlsAcceptor};

pub use data;
use self::data::{ChatMessage, ConnectedClient};

pub mod clienthandler;
use self::clienthandler::ClientHandler;

pub struct ChatServer {
    new_connections: websocket::sync::Server<NoTlsAcceptor>,
    pub chat_history: Vec<ChatMessage>,
    client_handlers: BTreeSet<ClientHandler>,
    new_messages: mpsc::Receiver<ChatMessage>,
    message_sender: mpsc::Sender<ChatMessage>,
}

impl ChatServer {
    pub fn new() -> Self {
        let address = dotenv!("MIN_CHAT_SERVER_ADDRESS").parse::<SocketAddrV4>()
            .expect("Could not parse server address");
        let new_connections = websocket::sync::Server::bind(address).unwrap();
        let (new_messages, message_sender) = mpsc::channel();
        ChatServer {
            new_connections,
            chat_history: Vec::new(),
            client_handlers: BTreeSet::new(),
            new_messages,
            message_sender,
        }
    }

    pub fn notify_handlers(&self, message: &ChatMessage) {
        for handler in &self.client_handlers {
            handler.notify_client(message);
        }
    }

    pub fn run(self) {
        for connection in self.new_connections.filter_map(Result::ok) {
            thread::spawn(move || {
                // let mut client = ClientHandler {
                //     history: self.chat_history,
                //     stream: connection.accept().unwrap()
                // };
                // println!("Accepted connection on {:?}", client.stream.local_addr().unwrap());
                // client.run();
            });
            let client_ws = connection.accept().unwrap();
            let (tx, rx) = mpsc::channel();
            // let new_handler = self.client_handlers.push(ClientHandler::new(client_ws, ))
        }
    }
}