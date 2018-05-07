#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate dotenv_codegen;

extern crate websocket;

mod chatserver;
use chatserver::ChatServer;

fn main() {
    ChatServer::launch();
}
