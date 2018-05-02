extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate dotenv_codegen;

extern crate websocket;

use std::net::{SocketAddrV4, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::{DateTime, UTC};

use websocket::server::{NoTlsAcceptor};
use websocket::OwnedMessage::{Text, Close};

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

pub struct ChatServer {
    listener: websocket::sync::Server<NoTlsAcceptor>,
    chat_history: Arc<Mutex<Vec<ChatMessage>>>,
}

impl ChatServer {
    pub fn new() -> Self {
        let address = dotenv!("MIN_CHAT_SERVER_ADDRESS").parse::<SocketAddrV4>()
            .expect("Could not parse server address");
        let listener = websocket::sync::Server::bind(address).unwrap();
        ChatServer { listener, chat_history: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn run(self) {
        println!("Listening on {:?}", self.listener.local_addr().unwrap());
        for connection in self.listener.filter_map(Result::ok) {
            let chat_history = self.chat_history.clone();
            thread::spawn(move || {
                let mut client = ClientHandler {
                    history: chat_history,
                    stream: connection.accept().unwrap()
                };
                println!("Accepted connection on {:?}", client.stream.local_addr().unwrap());
                client.run();
            });
        }
    }
}

pub struct ClientHandler {
    history: Arc<Mutex<Vec<ChatMessage>>>,
    stream: websocket::sync::Client<TcpStream>,
}

impl ClientHandler {
    pub fn send_all_history(&mut self) {
        for chat_msg in self.history.lock().unwrap().iter() {
            let string = serde_json::to_string(&chat_msg).unwrap();
            let msg = websocket::Message::text(string);
            self.stream.send_message(&msg).unwrap();
        }
    }


    pub fn run(&mut self) {
        self.send_all_history();
        loop {
            let msg = match self.stream.recv_message().unwrap() {
                Text(txt) => txt,
                Close(_) => return,
                _ => panic!("Recieved uninterpretable message!")
            };

            let chat_message: ChatMessage = serde_json::from_str(&msg).unwrap();
            println!("Recv'd {:?} from client", chat_message);
            let mut history = self.history.lock().unwrap();
            history.push(chat_message);
        }
    }
}

fn main() {
    let server = ChatServer::new();
    {
        let mut history = server.chat_history.lock().unwrap();
        history.push(ChatMessage {
            sender: "User1".to_owned(), body: "Hi there, how's it going?".to_owned(), timestamp: 1234123412
        });
        history.push(ChatMessage {
            sender: "User2".to_owned(), body: "It's going well!".to_owned(), timestamp: 1234123612
        });
    }
    server.run();
}
