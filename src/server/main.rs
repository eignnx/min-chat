extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate dotenv_codegen;

use std::io::Write;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::Arc;

use chrono::{DateTime, UTC};
use serde::{Serialize, Deserialize};
use serde_json::{Serializer, Deserializer};


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ConnectedClient {
    username: String,
    id: u64,
    // Color, Avatar, ...
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    sender: String,
    timestamp: u64,
    body: String
}

#[derive(Debug)]
pub struct ChatServer {
    listener: TcpListener,
    chat_history: Arc<Vec<ChatMessage>>,
}

impl ChatServer {
    pub fn new() -> Self {
        let address = dotenv!("MIN_CHAT_SERVER_ADDRESS").parse::<SocketAddrV4>().unwrap();
        let listener = TcpListener::bind(address).unwrap();
        ChatServer { listener, chat_history: Arc::new(Vec::new()) }
    }

    pub fn run(self) {
        println!("Listening on {:?}", self.listener.local_addr().unwrap());
        for mut stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Accepted connection on {:?}", stream.local_addr().unwrap());

            let mut msg = b"Hello is this working?\n";
            stream.write_all(&msg[..]);

            // Listen to stream, get username of connecting client.
            // let username: String = serde_json::from_reader(stream).unwrap();
            // let new_user = ConnectedClient {username, id: 12345};

            // // TODO: Put inside separate thread!
            // let handler = ClientHandler {
            //     server: self.chat_history.clone(), // Bumps reference count.
            //     stream
            // };

            // ...
        }
    }
}

#[derive(Debug)]
pub struct ClientHandler {
    server: Arc<Vec<ChatMessage>>,
    stream: TcpStream,
}

fn main() {
    let server = ChatServer::new();
    server.run();
}
