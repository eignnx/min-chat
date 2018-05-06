#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate dotenv_codegen;

extern crate websocket;

mod chatserver;
use chatserver::ChatServer;
use chatserver::data::ChatMessage;

fn main() {
    let server = ChatServer::new();
    server.chat_history.push(ChatMessage {
        sender: "User1".to_owned(), body: "Hi there, how's it going?".to_owned(), timestamp: 1234123412
    });
    server.chat_history.push(ChatMessage {
        sender: "User2".to_owned(), body: "It's going well!".to_owned(), timestamp: 1234123612
    });
    server.run();
}
