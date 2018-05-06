
use std::net::{TcpStream, SocketAddr};
use std::cmp::{PartialOrd, Ordering};
use data::{ConnectedClient, ChatMessage};
use websocket::OwnedMessage::{Text, Close};

use websocket;
use std::sync::mpsc;
use serde_json;

pub struct ClientHandler {
    // stream: websocket::sync::Client<TcpStream>,
    receiver: websocket::receiver::Reader<TcpStream>,
    sender: websocket::sender::Writer<TcpStream>,
    parent_channel: mpsc::Sender<ChatMessage>,
}

impl PartialEq for ClientHandler {
    fn eq(&self, other: &ClientHandler) -> bool {
        self.client_address() == other.client_address()
    }
}

impl Eq for ClientHandler {}

impl PartialOrd for ClientHandler {
    fn partial_cmp(&self, other: &ClientHandler) -> Option<Ordering> {
        self.client_address().partial_cmp(other.client_address())
    }
}

impl Ord for ClientHandler {
    fn cmp(&self, other: &ClientHandler) -> Ordering {
        self.client_address().cmp(other.client_address())
    }
}

impl ClientHandler {

    /// Returns the address of the client.
    pub fn client_address(&self) -> SocketAddr {
        self.sender.stream.peer_addr().unwrap()
    }

    pub fn new(client: websocket::sync::Client<TcpStream>, parent_channel: mpsc::Sender<ChatMessage>)
        -> ClientHandler
    {
        let (mut receiver, mut sender) = client.split().unwrap();
        let handler = ClientHandler { receiver, sender, parent_channel };
        handler.run();
        return handler;
    }

    pub fn send_all_history(&mut self) {
        unimplemented!();
    }

    pub fn notify_client(&self, message: &ChatMessage) {
        self.sender.send_message(message.into_message()).unwrap();
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

pub fn listen_to_client(client_ws: websocket::sync::Reader<TcpStream>) {
    for message in client_ws.incoming_messages() {

    }
}
